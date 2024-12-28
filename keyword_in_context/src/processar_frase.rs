use std::collections::HashSet;

/// # Parâmetros
/// - `frase`: Um vetor contendo as palavras a serem processadas.
/// - `stop_words_set`: Um HashSet contendo as stop words a serem ignoradas (HashSet tem complexidade O(1) para busca).
///
/// # Retorno
/// Um vetor de strings contendo as versões deslocadas da frase.
///
/// # Exemplo de Uso
/// ```
/// use std::collections::HashSet;
///
/// let frase = vec![
///       "O".to_string(),"projeto".to_string(),"de".to_string(),"ISC".to_string(),"é".to_string(),"de".to_string(),
///       "mais".to_string(),"para".to_string(),"o".to_string(),"primeiro".to_string(),"semestre".to_string(),];
///
/// let stop_words_set: HashSet<&str> = vec!["O", "de", "é", "mais", "para", "o"].into_iter().collect();
/// let resultado = processar_frase(frase, &stop_words_set);
/// ```
/// Resultado: [
///     "projeto | de ISC é de mais para o primeiro semestre O",
///     "ISC | é de mais para o primeiro semestre O projeto de",
///     "primeiro | semestre O projeto de ISC é de mais para",
///     "semestre | O projeto de ISC é de mais para o primeiro"
/// ];
fn processar_frase(frase: Vec<String>, stop_words_set: &HashSet<String>) -> Vec<String> {
    let mut frases_deslocadas = Vec::new();

    for (indice, palavra) in frase.iter().enumerate() {
        let palavra_sem_pontuacao: String = palavra.chars().filter(|c| c.is_alphanumeric()).collect();

        if !stop_words_set.contains(palavra_sem_pontuacao.as_str()) {
            let frase_deslocada = deslocamento_circular(indice, &frase);
            frases_deslocadas.push(frase_deslocada);
        }
    }

    frases_deslocadas
}
