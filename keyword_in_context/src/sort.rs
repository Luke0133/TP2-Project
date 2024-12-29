/// # Parameters
/// - `phrases: &mut Vec<String>`: Mutable reference to string vector containing all sentences to be ordered.
/// - `sort_type: &String`: reference to string containing sorting type
///
/// # Return
/// Result<(), String>
///

pub fn sort_phrases(phrases: &mut Vec<String>, sort_type: &String) -> Result<(), String> {
    if phrases.is_empty() {
        return Err(String::from("Vector Must not be empty\n"));
    }

    match sort_type.as_str(){
        "alpha" => {
            // Case insensitive, sorting alphabetically
            phrases.sort_by(|left, right| {
                let left_no_pipe = left.replace("|", "").to_lowercase();
                let right_no_pipe = right.replace("|", "").to_lowercase();
                left_no_pipe.cmp(&right_no_pipe)
            });
        }
        "sensitive" => {
            // Case sensitive, sorting alphabetically
            phrases.sort_by(|left, right| {
                let left_no_pipe = left.replace("|", "");
                let right_no_pipe = right.replace("|", "");
                left_no_pipe.cmp(&right_no_pipe)
            });
        }
        "reverse_alpha" => {
            // Reverse alphabetical order, case insensitive
            phrases.sort_by(|left, right| {
                let left_no_pipe = left.replace("|", "").to_lowercase();
                let right_no_pipe = right.replace("|", "").to_lowercase();
                right_no_pipe.cmp(&left_no_pipe) // Reverse order
            });
        }
        "reverse_sensitive" => {
            // Reverse alphabetical order, case sensitive
            phrases.sort_by(|left, right| {
                let left_no_pipe = left.replace("|", "");
                let right_no_pipe = right.replace("|", "");
                right_no_pipe.cmp(&left_no_pipe) // Reverse order
            });
        }
        _=> return Err(format!("{} is an invalid sort option.",sort_type))
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::sort;
    #[test]
    fn test_sort_alpha() {
        let mut phrases = vec![
            String::from("closet | A cat hid in the"),
            String::from("cat hid in the closet | A"),
            String::from("cat | A dog ran after the"),
            String::from("dog ran after the cat | A"),
            String::from("parrot sings when it wants. | The"),
            String::from("sings when it wants. | The parrot"),
            String::from("Grapes are great!"),
            String::from("great! | Grapes are "),
        ];
        let sort_type = "alpha".to_string();
        match sort::sort_phrases(&mut phrases,&sort_type) {
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
                assert_eq!(phrases, expected);
            }
            Err(s) => assert!(false, "{}", s),
        };
    }

    #[test]
    fn test_sort_sensitive() {
        let mut phrases = vec![
            String::from("closet | A cat hid in the"),
            String::from("cat hid in the closet | A"),
            String::from("cat | A dog ran after the"),
            String::from("dog ran after the cat | A"),
            String::from("parrot sings when it wants. | The"),
            String::from("sings when it wants. | The parrot"),
            String::from("Grapes are great!"),
            String::from("great! | Grapes are "),
        ];
        let sort_type = "sensitive".to_string();
        match sort::sort_phrases(&mut phrases,&sort_type) {
            Ok(()) => {
                let expected = vec![
                    String::from("Grapes are great!"),
                    String::from("cat | A dog ran after the"),
                    String::from("cat hid in the closet | A"),
                    String::from("closet | A cat hid in the"),
                    String::from("dog ran after the cat | A"),
                    String::from("great! | Grapes are "),
                    String::from("parrot sings when it wants. | The"),
                    String::from("sings when it wants. | The parrot"),
                ];
                assert_eq!(phrases, expected);
            }
            Err(s) => assert!(false, "{}", s),
        };
    }

    #[test]
    fn test_sort_reverse_alpha() {
        let mut phrases = vec![
            String::from("closet | A cat hid in the"),
            String::from("cat hid in the closet | A"),
            String::from("cat | A dog ran after the"),
            String::from("dog ran after the cat | A"),
            String::from("parrot sings when it wants. | The"),
            String::from("sings when it wants. | The parrot"),
            String::from("Grapes are great!"),
            String::from("great! | Grapes are "),
        ];
        let sort_type = "reverse_alpha".to_string();
        match sort::sort_phrases(&mut phrases,&sort_type) {
            Ok(()) => {
                let expected = vec![
                    String::from("sings when it wants. | The parrot"),
                    String::from("parrot sings when it wants. | The"),
                    String::from("great! | Grapes are "),
                    String::from("Grapes are great!"),
                    String::from("dog ran after the cat | A"),
                    String::from("closet | A cat hid in the"),
                    String::from("cat hid in the closet | A"),
                    String::from("cat | A dog ran after the"),
                ];
                assert_eq!(phrases, expected);
            }
            Err(s) => assert!(false, "{}", s),
        };
    }

    #[test]
    fn test_sort_reverse_sensitive() {
        let mut phrases = vec![
            String::from("great! | Grapes are "),
            String::from("sings when it wants. | The parrot"),
            String::from("parrot sings when it wants. | The"),
            String::from("dog ran after the cat | A"),
            String::from("cat | A dog ran after the"),
            String::from("cat hid in the closet | A"),
            String::from("closet | A cat hid in the"),
            String::from("Grapes are great!"),
        ];
        let sort_type = "reverse_sensitive".to_string();
        match sort::sort_phrases(&mut phrases,&sort_type) {
            Ok(()) => {
                let expected = vec![
                    String::from("sings when it wants. | The parrot"),
                    String::from("parrot sings when it wants. | The"),
                    String::from("great! | Grapes are "),
                    String::from("dog ran after the cat | A"),
                    String::from("closet | A cat hid in the"),
                    String::from("cat hid in the closet | A"),
                    String::from("cat | A dog ran after the"),
                    String::from("Grapes are great!"),
                ];
                assert_eq!(phrases, expected);
            }
            Err(s) => assert!(false, "{}", s),
        };
    }

    #[test]
    // Evaluates error when passing empty vector
    fn test_sort_error_empty() {
        let mut phrases = vec![];
        let sort_type = "sensitive".to_string();
        match sort::sort_phrases(&mut phrases,&sort_type) {
            Ok(()) => assert!(false, "Unexpected Error\n"),
            Err(s) => assert!(true, "{}", s),
        };
    }

    #[test]
    // Evaluates error when passing invalid sorting option
    fn test_sort_error_sort() {
        let mut phrases = vec![
            String::from("great! | Grapes are "),
            String::from("sings when it wants. | The parrot"),
            String::from("parrot sings when it wants. | The"),
            String::from("dog ran after the cat | A"),
            String::from("cat | A dog ran after the"),
            String::from("cat hid in the closet | A"),
            String::from("closet | A cat hid in the"),
            String::from("Grapes are great!"),
        ];
        let sort_type = "not_an_option_:D".to_string();
        match sort::sort_phrases(&mut phrases,&sort_type) {
            Ok(()) => assert!(false, "Unexpected Error\n"),
            Err(s) => assert!(true, "{}", s),
        };
    }

}
