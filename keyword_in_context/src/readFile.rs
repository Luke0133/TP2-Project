use std::fs;

/// # Parâmetros
/// - `file`: Uma string contendo o nome do arquivo dentro de src a ser processado.
///
/// # Retorno
/// Um vetor de vetores de strings contendo as linhas do arquivo separadas por espacos.
///
/// # Exemplo de Uso
/// ``
/// use std::fs;
///
/// let s = reading("readtest.txt");
/// println!("{:?}", s);
/// ``
/// Resultado: [["I'm", "nobody!", "Who", "are", "you?"], 
/// ["Are", "you", "nobody,", "too?"], 
/// ["Then", "there's", "a", "pair", "of", "us", "-", "don't", "tell!"], 
/// ["They'd", "banish", "us,", "you", "know."], 
/// ["How", "dreary", "to", "be", "somebody!"], 
/// ["How", "public,", "like", "a", "frog"], 
/// ["To", "tell", "your", "name", "the", "livelong", "day"], 
/// ["To", "an", "admiring", "bog!"]];


fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    if s.ends_with(p) {
        &s[..s.len() - p.len()]
    } else {
        s
    }
}

pub fn reading(file: &str) -> Vec<Vec<String>> {
    let data = fs::read_to_string(file).expect("Unable to read file! I quit!");
    let parts = data.split("\n");
    let aux: Vec<String> = parts.map(String::from).collect();
    let mut linhas: Vec<Vec<String>> = Vec::new();
    if aux == [""]{
        println!("File is empty! I quit!");
        return linhas;
    }
    for linha in aux {
        let s = remove_suffix(&linha, "\r");
        let s2: Vec<String> = (s.split(" ")).map(String::from).collect();
        if s2 != [""]{
            linhas.push(s2);
        }
    }
    linhas
}
