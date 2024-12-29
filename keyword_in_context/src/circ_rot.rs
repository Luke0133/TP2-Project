pub fn deslocamento_circular(indice: usize, palavras: Vec<String>) -> Result<String, String> {
    if palavras.is_empty() {
        return Err("Não dá para procurar uma palavra em um vetor que não tem palavras!".to_string());
    }
    let mut deslocada = palavras;
    if indice != 0{
        deslocada.push("|".to_string());
        deslocada.rotate_left(indice);
    }
    let deslocada2: String = deslocada.join(" ");
    Ok(deslocada2)
}

#[cfg(test)]
mod tests {
    use crate::circ_rot;
    #[test]
    fn test_deslocamento1(){
        let palavras = vec![
            "A".to_string(), 
            "cat".to_string(), 
            "hid".to_string(), 
            "in".to_string(), 
            "the".to_string(),
            "cat".to_string(), 
            "closet".to_string(),
        ];
        let indice: usize = 5;
        let resultado = circ_rot::deslocamento_circular(indice, palavras).unwrap();
        let correto = "cat closet | A cat hid in the";
        assert_eq!(resultado, correto);
    }

    #[test]
    fn test_deslocamento2(){
        let palavras = vec![
            "A".to_string(), 
            "cat".to_string(), 
            "hid".to_string(), 
            "in".to_string(), 
            "the".to_string(),
            "cat".to_string(), 
            "closet".to_string(),
        ];
        let indice: usize = 1;
        let resultado = circ_rot::deslocamento_circular(indice, palavras).unwrap();
        let correto = "cat hid in the cat closet | A";
        assert_eq!(resultado, correto);
    }
    #[test]
    fn test_vetor_vazio() {
        let palavras: Vec<String> = vec![];
        let indice: usize = 1;
        let resultado = circ_rot::deslocamento_circular(indice, palavras);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "Não dá para procurar uma palavra em um vetor que não tem palavras!");
    }
}