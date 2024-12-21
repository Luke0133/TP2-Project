use std::collections::HashSet;

/// # Parâmetros
/// - `frase`: Uma string contendo a frase a ser processada.
/// - `stop_words_set`: Um HashSet contendo as stop words a serem ignoradas (HashSet tem complexidade O(1) para busca).
///
/// # Retorno
/// Um vetor de strings contendo as versões deslocadas da frase.
///
/// # Exemplo de Uso
/// ```
/// use std::collections::HashSet;
///
/// let frase = "O projeto de ISC é de mais para o primeiro semestre.";
/// let stop_words_set: HashSet<&str> = vec!["O", "de", "é", "mais", "para", "o"].into_iter().collect();
/// let resultado = processar_frase(frase, &stop_words_set);
/// ```
/// Resultado: [
///     "projeto | de ISC é de mais para o primeiro semestre O",
///     "ISC | é de mais para o primeiro semestre O projeto de",
///     "primeiro | semestre O projeto de ISC é de mais para",
///     "semestre | O projeto de ISC é de mais para o primeiro"
/// ];
fn processar_frase(frase: &str, stop_words_set: &HashSet<&str>) -> Vec<String> {
    let mut frases_deslocadas = Vec::new();

    let palavras: Vec<&str> = frase 
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .filter(|s| !s.is_empty())
        .collect();

    for palavra in &palavras {

        if !stop_words_set.contains(*palavra) {
            let frase_deslocada = deslocamento_circular(palavra, &palavras);
            frases_deslocadas.push(frase_deslocada);
            }
        }
    }

    frases_deslocadas
}