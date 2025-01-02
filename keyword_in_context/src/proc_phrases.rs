use std::collections::HashSet;
use crate::circ_rot;

/// 
/// 
/// # Parameters
/// - `frase: Vec<String>`: Vector representing a sentence of strings
/// - `stop_words_set: &HashSet<String>`: imutable reference to HashSet of stop words
/// - `length: Option<i32>`: length of window arround non-stop words
/// # Returns
/// - Result<Vec<String>, String>, where the the vector contains all rotated phrases
///

pub fn processar_frase(frase: Vec<String>, stop_words_set: &HashSet<String>, length: Option<i32>) -> Result<Vec<String>, String> {

    if frase.is_empty() {
        return Err("The sentences vector must not be empty.".to_string());
    }
    if stop_words_set.is_empty() {
        return Err("The HashSet 'stop_words_set' must not be empty.".to_string());
    }
    
    let mut frases_deslocadas = Vec::new();

    for (indice, palavra) in frase.iter().enumerate() {
        let palavra_sem_pontuacao: String = palavra.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();

        if !stop_words_set.contains(palavra_sem_pontuacao.as_str()) {
            let frase_deslocada = circ_rot::deslocamento_circular(indice, length,frase.clone());
            frases_deslocadas.push(frase_deslocada?);
        }
    }

    Ok(frases_deslocadas)
}

#[cfg(test)]
mod tests {
    use crate::proc_phrases;
    use std::collections::HashSet;
    #[test]
    // Tests when there are no arguments
    fn proc_phrases_test() {
        let frase = vec![
            "O".to_string(),"projeto".to_string(),"de".to_string(),"ISC".to_string(),"é".to_string(),"de".to_string(),
            "mais".to_string(),"para".to_string(),"o".to_string(),"primeiro".to_string(),"semestre".to_string(),];

        let stop_words_set: HashSet<String> = vec!["O", "de", "é", "mais", "para", "o"].into_iter().map(|s| s.to_string()).collect();
        let lenght = None;

        match proc_phrases::processar_frase(frase, &stop_words_set,lenght)  {
            Ok(vector) => {
                let expected = vec![
                    "projeto de ISC é de mais para o primeiro semestre | O",
                    "ISC é de mais para o primeiro semestre | O projeto de",
                    "primeiro semestre | O projeto de ISC é de mais para o",
                    "semestre | O projeto de ISC é de mais para o primeiro"
                ];
                assert_eq!(vector, expected);
            }
            Err(s) => assert!(false, "{}", s),
        };
    }
}

