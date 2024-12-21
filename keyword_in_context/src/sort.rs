pub fn sort_phrases(phrases: &mut Vec<String>) -> Result<(), String> {
    if phrases.is_empty() {
        return Err(String::from("Vector Must not be empty"));
    }

    phrases.sort_by(|left, right| {
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

        match sort::sort_phrases(&mut phrases) {
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
}
