/// # Parâmetros
/// - `frases`: Um vetor (mutável) de strings (as frases).
///
/// # Retorno
/// Result contendo Ok() vazio (dado que o vetor de parâmetro é mutável) ou Err(String) com a mensagem de erro.
///

pub fn sort_phrases(frases: &mut Vec<String>) -> Result<(), String> {
    if frases.is_empty() {
        return Err(String::from("Vector Must not be empty\n"));
    }

    frases.sort_by(|left, right| {
        let left_no_pipe = left.replace("|", "").to_lowercase();
        let right_no_pipe = right.replace("|", "").to_lowercase();
        left_no_pipe.cmp(&right_no_pipe)
    });
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::sort;
    #[test]
    fn test_sort() {
        let mut frases = vec![
            String::from("closet!  A cat hid in the"),
            String::from("cat hid in the closet!  A"),
            String::from("cat  A dog ran after the"),
            String::from("dog ran after the cat  A"),
            String::from("parrot sings when it wants.  The"),
            String::from("sings when it wants.  The parrot"),
            String::from("Grapes are great!"),
            String::from("great!  Grapes are"),
        ];

        match sort::sort_phrases(&mut frases) {
            Ok(()) => {
                let expected = vec![
                    String::from("cat  A dog ran after the"),
                    String::from("cat hid in the closet!  A"),
                    String::from("closet!  A cat hid in the"),
                    String::from("dog ran after the cat  A"),
                    String::from("Grapes are great!"),
                    String::from("great!  Grapes are"),
                    String::from("parrot sings when it wants.  The"),
                    String::from("sings when it wants.  The parrot"),
                ];
                assert_eq!(frases, expected);
            }
            Err(s) => assert!(false, "{}", s),
        };
    }

    #[test]
    fn test_sort_pipe() {
        let mut frases = vec![
            String::from("closet | A cat hid in the"),
            String::from("cat hid in the closet | A"),
            String::from("cat | A dog ran after the"),
            String::from("dog ran after the cat | A"),
            String::from("parrot sings when it wants. | The"),
            String::from("sings when it wants. | The parrot"),
            String::from("Grapes are great!"),
            String::from("great! | Grapes are "),
        ];

        match sort::sort_phrases(&mut frases) {
            Ok(()) => {
                let expected = vec![
                    String::from("cat | A dog ran after the"),
                    String::from("cat hid in the closet | A"),
                    String::from("closet | A cat hid in the"),
                    String::from("dog ran after the cat | A"),
                    String::from("Grapes are great!"),
                    String::from("great! | Grapes are "),
                    String::from("parrot sings when it wants. | The"),
                    String::from("sings when it wants. | The parrot"),
                ];
                assert_eq!(frases, expected);
            }
            Err(s) => assert!(false, "{}", s),
        };
    }

    #[test]
    // Evaluates error
    fn test_sort_error() {
        let mut frases = vec![];

        match sort::sort_phrases(&mut frases) {
            Ok(()) => assert!(false, "Unexpected Error\n"),
            Err(s) => assert!(true, "{}", s),
        };
    }

}
