pub fn deslocamento_circular(indice: usize, redor: Option<i32>, palavras: Vec<String>) -> Result<String, String> {
    let mut index = indice;
    if palavras.is_empty() {
        return Err("Vector Must not be empty!".to_string());
    }
    let mut deslocada = palavras;
    let tamanho_frase = deslocada.len();
    if index >= tamanho_frase {
        return Err("Index outside of vector's range!".to_string());
    }
    
    let mut passou_esquerda = false;
    let mut passou_direita = false;
    if redor != None  {
        let valor_redor = redor.unwrap();
        let mut index_esquerda = index as i32 - valor_redor;
        if index_esquerda < 0 {
            index_esquerda += tamanho_frase as i32;
            passou_esquerda = true;
        }
        let index_esquerda = index_esquerda as usize;

        let mut index_direita = index + valor_redor as usize;
        if index_direita >= tamanho_frase {
            index_direita -= tamanho_frase;
            passou_direita = true;
        }

        if passou_esquerda && passou_direita {
            return Err(format!("Given window value of {} was too big for some sentences such as: {}",valor_redor,deslocada.join(" ")));
        }
        if passou_direita && index_direita >= index_esquerda {
            return Err(format!("Given window value of {} was too big for some sentences such as: {}",valor_redor,deslocada.join(" ")))
        }
        if passou_esquerda && index_esquerda <= index_direita {
            return Err(format!("Given window value of {} was too big for some sentences such as: {}",valor_redor,deslocada.join(" ")))
        }

        if (index_esquerda as i32 - index_direita as i32).abs() > 1 {
            let lado_esquerdo = if passou_esquerda {
            let mut parte1 = deslocada[index_esquerda..].to_vec();
            let mut parte2 = deslocada[..index].to_vec();
            parte1.append(&mut parte2);
            parte1
            } else {
                deslocada[index_esquerda..index].to_vec()
            };

            let lado_direito = if passou_direita {
                let mut parte1 = deslocada[index + 1..].to_vec();
                let mut parte2 = deslocada[..index_direita + 1].to_vec();
                parte1.append(&mut parte2);
                parte1
            } else {
                deslocada[index + 1..=index_direita].to_vec()
            };
            
            let palavra_chave = deslocada[index].clone();
            deslocada = lado_esquerdo.clone();
            index = deslocada.len();
            deslocada.push(palavra_chave);
            deslocada.extend(lado_direito.clone());
        }
    }

    if index != 0{
        deslocada.push("|".to_string());
        deslocada.rotate_left(index);
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
        let janela = None;
        let resultado = circ_rot::deslocamento_circular(indice, janela, palavras).unwrap();
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
        let janela = None;
        let resultado = circ_rot::deslocamento_circular(indice, janela, palavras).unwrap();
        let correto = "cat hid in the cat closet | A";
        assert_eq!(resultado, correto);
    }
    #[test]
    fn test_vetor_vazio() {
        let palavras: Vec<String> = vec![];
        let indice: usize = 1;
        let janela = None;
        match circ_rot::deslocamento_circular(indice, janela, palavras){
            Ok(s) => assert!(false, "An error ocurred, the program should've failed: answer was {}", s),
            Err(s) => assert!(true, "{}", s),
        }
    }
    #[test]
    fn test_deslocamento3(){
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
        let janela = Some(1);
        let resultado = circ_rot::deslocamento_circular(indice, janela, palavras).unwrap();
        let correto = "cat closet | the";
        assert_eq!(resultado, correto);
    }

    #[test]
    fn test_deslocamento4(){
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
        let janela = Some(3);
        let resultado = circ_rot::deslocamento_circular(indice, janela, palavras).unwrap();
        let correto = "cat closet | A cat hid in the";
        assert_eq!(resultado, correto);
    }

    #[test]
    fn test_deslocamento5(){
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
        let janela = Some(2);
        let resultado = circ_rot::deslocamento_circular(indice, janela, palavras).unwrap();
        let correto = "cat hid in | closet A";
        assert_eq!(resultado, correto);
    }

}