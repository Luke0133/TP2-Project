use std::fs;
use std::path::Path;

/// # Parameters
/// - `file`: Uma string contendo o nome do arquivo dentro de src a ser processado.
///
/// # Returns
/// - Result<Vec<Vec<String>>, String>, where the the vector contains a vectors which are the sentences divided by word 
///
 
fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    if s.ends_with(p) {
        &s[..s.len() - p.len()]
    } else {
        s
    }
}

pub fn get_phrases(path: String) -> Result<Vec<Vec<String>>, String> {
    let file: &str = path.as_str();
    if Path::new(file).exists() == false{
        return Err(String::from("File doesn't exist! I quit!"));
    }
    let data = fs::read_to_string(file).expect("Unable to read file! I quit!");
    let parts = data.split("\n");
    let aux: Vec<String> = parts.map(String::from).collect();
    let mut linhas: Vec<Vec<String>> = Vec::new();
    if aux == [""]{
        return Err(String::from("File is empty! I quit!"));
    }
    for linha in aux {
        let s = remove_suffix(&linha, "\r");
        let s2: Vec<String> = (s.split(" ")).map(String::from).collect();
        if s2 != [""]{
            linhas.push(s2);
        }
    }
    Ok(linhas)
}

#[cfg(test)]
mod tests {
    use crate::read_phrase_files;
    #[test]
    fn test_get_phrases() {
        let path = "./files/readtest.txt".to_string(); //O uso das barras desse modo se dá pelo fato que os testes foram feitos em Windows
        
        match read_phrase_files::get_phrases(path) {
            Ok(linhas) => {
                let expected: Vec<Vec<String>> = vec![
                ["I'm".to_string(), "nobody!".to_string(), "Who".to_string(), "are".to_string(), "you?".to_string()].to_vec(),
                ["Are".to_string(), "you".to_string(), "nobody,".to_string(), "too?".to_string()].to_vec(),
                ["Then".to_string(), "there's".to_string(), "a".to_string(), "pair".to_string(), "of".to_string(), "us".to_string(), "-".to_string(), "don't".to_string(), "tell!".to_string()].to_vec(), 
                ["They'd".to_string(), "banish".to_string(), "us,".to_string(), "you".to_string(), "know.".to_string()].to_vec(), 
                ["How".to_string(), "dreary".to_string(), "to".to_string(), "be".to_string(), "somebody!".to_string()].to_vec(), 
                ["How".to_string(), "public,".to_string(), "like".to_string(), "a".to_string(), "frog".to_string()].to_vec(), 
                ["To".to_string(), "tell".to_string(), "your".to_string(), "name".to_string(), "the".to_string(), "livelong".to_string(), "day".to_string()].to_vec(), 
                ["To".to_string(), "an".to_string(), "admiring".to_string(), "bog!".to_string()].to_vec()];
                assert_eq!(linhas, expected);
            }
            Err(s) => assert!(false, "{}", s),
        };
    }
    #[test]
    fn test_get_phrases_fail1() {
        let path = "./files/empty.txt".to_string(); //O uso das barras desse modo se dá pelo fato que os testes foram feitos em Windows

        match read_phrase_files::get_phrases(path) {
            Ok(_linhas) => assert!(false, ""),
            Err(s) => assert!(true, "{}", s),
        };
    }
    #[test]
    fn test_get_phrases_fail2() {
        let path = "./files/nonexistent.txt".to_string(); //O uso das barras desse modo se dá pelo fato que os testes foram feitos em Windows

        match read_phrase_files::get_phrases(path) {
            Ok(_linhas) => assert!(false, ""),
            Err(s) => assert!(true, "{}", s),
        };
    }
}