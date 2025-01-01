use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Reads a file containing stop words and creates a HashSet with them, for an easy lookup
/// 
/// # Parameters
/// - `path: P`: Path to file containing stop words
/// 
/// # Returns
/// - Result<HashSet<String>, String>, where HashSet contains the file's stop words
/// 
pub fn get_stop_words<P: AsRef<Path>>(path: P) -> Result<HashSet<String>, String> {
    let mut stopwords = HashSet::new();
    
    let file = File::open(path).map_err(|e| format!("Error when opening file: {}", e))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line.map_err(|e| format!("Error reading line: {}", e))?.trim().to_lowercase();
        if !word.is_empty() {
            stopwords.insert(word);
        }
    }

    if stopwords.is_empty() {
        return Err("File is empty! I quit!".to_string());
    }
    
    Ok(stopwords)
}

/// Verifica se uma palavra é uma stop word
/// 
/// # Parameters
/// - `word: &str`: Palavra a ser verificada
/// - `stop_words_set: &HashSet<String>`: Referência imutável para o HashSet de stop words
/// 
/// # Returns
/// - bool: true se a palavra é uma stop word, false caso contrário
/// 
pub fn is_stopword(word: &str, stop_words_set: &HashSet<String>) -> bool {
    let word_normalized: String = word.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    stop_words_set.contains(&word_normalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use std::fs;

    #[test]
    fn test_leitura_stopwords() {
        let temp_path = "./files/testando_stopwords.txt";
        let content = "the\nand\nin\n";
        write(temp_path, content).unwrap();
        
        match get_stop_words(temp_path) {
            Ok(stopwords) => {
                assert_eq!(stopwords.len(), 3);
                assert!(stopwords.contains("the"));
                assert!(stopwords.contains("and"));
                assert!(stopwords.contains("in"));
            },
            Err(e) => assert!(false, "Erro inesperado: {}", e),
        }
        
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_is_stopword() {
        let mut stop_words_set = HashSet::new();
        stop_words_set.insert("the".to_string());
        stop_words_set.insert("and".to_string());
        
        assert!(is_stopword("THE", &stop_words_set));
        assert!(is_stopword("and", &stop_words_set));
        assert!(!is_stopword("hello", &stop_words_set));
        assert!(is_stopword("The!", &stop_words_set)); // Testa remoção de pontuação
    }
}