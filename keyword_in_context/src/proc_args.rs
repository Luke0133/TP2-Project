/// # Parameters
/// - `args: &Vec<String>`: Reference to String vector that contains user's arguments
/// - `phrases_file: &mut String`: Mutable reference to String that contains the sentences .txt file path
/// - `stop_words_file: &mut String`: Mutable reference to String that contains the stop words .txt file path
/// - `context_len: &mut Option<i32>`: Mutable reference to option that contains context range for wrapping sentences
/// - `sort_type: &mut String`: Mutable reference to String that contains the sorting option
///
/// # Returns
/// - Result<bool,String>, where bool will have value of true if -h or --help was passed as an argument
///

pub fn process_args(args: &Vec<String>, phrases_file: &mut String, 
    stop_words_file: &mut String, context_len: &mut Option<i32>, sort_type: &mut String)-> Result<bool,String> {

    let argc = args.len();
    if argc <= 1 {
        return Ok(false)
    }

    if argc > 2 && args[1].as_str() == "args" { 
        // Iterating through arguments
        let mut i = 2;
        while i < argc {
            match args[i].as_str() {
                "-h" | "--help" => {
                    println!("Usage: 'cargo run' or 'cargo run args' followed by arguments (in any order):
|   (optional) -h or --help: help with how to run code
|   (optional) -p or --phrases: followed by a .txt with sentences to be used (uses ./files/sentences.txt as default)
|   (optional) -w or --words: followed by a .txt with stop words to be used (uses ./files/stop_words.txt as default)
|   (optional) -l or --length: followed by the number of words to be printed around words in context (defaults to wraping the whole sentence),
|                              and if num is bigger than a given sentence, it defaults to wraping around the whole sentence
|   (optional) -s or --sort: followed by 'alpha' to sort in alphabetic order, 'sensitive' for a case-sensitive sort,
|                            'reverse_alpha' for a reverse alphabetic order or 'reverse sensitive' for a reverse case-sensitive sort
|                            (defaults to 'alpha')\n");
                    return Ok(true)
                }
                "-p" | "--phrases" => {
                    if i + 1 < argc {
                        if args[i + 1].starts_with("-") {
                            return Err(format!("{} expects a file path afterwards not a flag. If {} is a filename, remove the '-'", args[i],args[i + 1]));
                        }
                        *phrases_file = args[i + 1].clone();
                        i += 1; //skips next argument (will be the value of phrases_file)
                    } else {
                        return Err(format!("{} expects a file path afterwards and none was found.",args[i]));
                    }      
                }
                "-w" | "--words" => {
                    if i + 1 < argc {
                        if args[i + 1].starts_with("-") {
                            return Err(format!("{} expects a file path afterwards not a flag. If {} is a filename, remove the '-'", args[i],args[i + 1]));
                        }
                        *stop_words_file = args[i + 1].clone();
                        i += 1; //skips next argument (will be the value of stop_words_file)
                    } else {
                        return Err(format!("{} expects a file path afterwards and none was found.",args[i]));
                    }     
                }
                "-l" | "--length" => {
                    if i + 1 < argc {
                        if args[i + 1].starts_with("-") {
                            return Err(format!("{} expects an integer afterwards not a flag.", args[i]));
                        }
                        let num: Result<i32, _> = args[i + 1].parse().map_err(|_| format!("{} expects an integer afterwards.",args[i]));
                        *context_len = Some(num?);
                        i += 1; //skips next argument (will be the value of stop_words_file)
                    } else {
                        return Err(format!("{} expects an integer afterwards and none was found.",args[i]));
                    }     
                }
                "-s" | "--sort" => {
                    if i + 1 < argc {
                        if args[i + 1].starts_with("-") {
                            return Err(format!("{} expects a sort option afterwards and not a flag. To see sort options type 'cargo run -h' or 'cargo run --help'", args[i]));
                        }
                        let valid_sort_values = ["alpha", "sensitive", "reverse_alpha", "reverse_sensitive"];

                        if valid_sort_values.contains(&args[i + 1].as_str()) {
                            *sort_type = args[i + 1].clone();
                            i += 1; // Skip the next argument (already used)
                        } else {
                            return Err(format!("{} is an invalid sort option. To see valid sort options type 'cargo run -h' or 'cargo run --help'", args[i + 1]));
                        }

                    } else {
                        return Err(format!("{} expects a sort option afterwards and none was found.",args[i]));
                    }     
                }
                _=> return Err(format!("'{}' is not a valid argument. Try typing cargo run -h or cargo run --help to see valid syntax.", args[i])),
            }
            i += 1;
        }
    } else {
        if args[1].as_str() == "args" {
            return Err("'args' needs to be followed by at least another argument.".to_string());
        }
        return Err(format!("'{}' is not a valid argument.", args[1]));
    }
    Ok(false)
    
}

#[cfg(test)]
mod tests {
    use crate::proc_args;
    #[test]
    // Tests when there are no arguments
    fn test_no_args() {
        let mut phrases_file: String = "./files/sentences.txt".to_string();          
        let mut stop_words_file: String = "./files/stop_words.txt".to_string();      
        let mut context_len: Option<i32> = None;        
        let mut sort_type: String = "alpha".to_string();    
        
        let args = vec![
            "target\\debug\\keyword_in_context.exe".to_string(),
        ];

        match proc_args::process_args(&args,&mut phrases_file, &mut stop_words_file, &mut context_len, &mut sort_type) {
            Ok(true) => assert!(false, "An error ocurred, help was not called"),
            Ok(false) => {
                // Nothing should be updated
                assert_eq!(phrases_file, "./files/sentences.txt".to_string());
                assert_eq!(stop_words_file, "./files/stop_words.txt".to_string());
                assert_eq!(context_len, None);
                assert_eq!(sort_type, "alpha".to_string());
            }
            Err(s) => assert!(false, "{}", s),
        };
    }

    #[test]
    // Tests when --help is called (assumes other syntax such as -h works as well)
    fn test_help() {
        let mut phrases_file: String = "./files/sentences.txt".to_string();          
        let mut stop_words_file: String = "./files/stop_words.txt".to_string();      
        let mut context_len: Option<i32> = None;        
        let mut sort_type: String = "alpha".to_string();    
        
        let args = vec![
            "target\\debug\\keyword_in_context.exe".to_string(),
            "args".to_string(),
            "--help".to_string(),
        ];

        match proc_args::process_args(&args,&mut phrases_file, &mut stop_words_file, &mut context_len, &mut sort_type) {
            Ok(true) => assert!(true, "Help was called sucessfully"),
            Ok(false) => assert!(false, "An error ocurred, help returns Ok(true)"),
            Err(s) => assert!(false, "{}", s),
        };
    }

    #[test]
    // Tests when various arguments are given (assumes other ways to write said flags are working)
    fn test_all_but_help() {
        let mut phrases_file: String = "./files/sentences.txt".to_string();          
        let mut stop_words_file: String = "./files/stop_words.txt".to_string();      
        let mut context_len: Option<i32> = None;        
        let mut sort_type: String = "alpha".to_string(); 

        let args = vec![
            "target\\debug\\keyword_in_context.exe".to_string(),
            "args".to_string(),
            "-p".to_string(),
            "./files/empty.txt".to_string(), 
            "--sort".to_string(),
            "reverse_alpha".to_string(),
            "-w".to_string(),
            "./files/none.txt".to_string(), 
            "--length".to_string(),
            "4".to_string()
        ];

        match proc_args::process_args(&args,&mut phrases_file, &mut stop_words_file, &mut context_len, &mut sort_type) {
            Ok(true) => assert!(false, "An error ocurred, help was not called"),
            Ok(false) => {
                // Check that the phrases_file, stop_words_file, etc. have been correctly updated
                assert_eq!(phrases_file, "./files/empty.txt".to_string());  // The correct path from `-p`
                assert_eq!(stop_words_file, "./files/none.txt".to_string());  // The correct path from `-w`
                assert_eq!(context_len, Some(4));  // The correct number from `--length`
                assert_eq!(sort_type, "reverse_alpha".to_string());  // The correct sort type from `--sort`
            }
            Err(s) => assert!(false, "{}", s),
        };
    }

    #[test]
    // Checks error where 'args' is called, but there are no following arguments
    fn test_no_args_error() {
        let mut phrases_file: String = "./files/sentences.txt".to_string();          
        let mut stop_words_file: String = "./files/stop_words.txt".to_string();      
        let mut context_len: Option<i32> = None;        
        let mut sort_type: String = "alpha".to_string();    
        
        let args = vec![
            "target\\debug\\keyword_in_context.exe".to_string(),
            "args".to_string()
        ];

        match proc_args::process_args(&args,&mut phrases_file, &mut stop_words_file, &mut context_len, &mut sort_type) {
            Ok(true) => assert!(false, "An error ocurred, help was not called"),
            Ok(false) => assert!(false, "No propper argument was given"),
            Err(s) => assert!(true, "{}", s),
        };
    }

    #[test]
    // Checks a case where incorrect syntax (missing argument) happens
    fn test_incorrect_args() {
        let mut phrases_file: String = "./files/sentences.txt".to_string();          
        let mut stop_words_file: String = "./files/stop_words.txt".to_string();      
        let mut context_len: Option<i32> = None;        
        let mut sort_type: String = "alpha".to_string(); 

        let args = vec![
            "target\\debug\\keyword_in_context.exe".to_string(),
            "args".to_string(),
            "-p".to_string(),      // There should be an error here, since there's no string following '-p' 
            "-l".to_string(),
            "4".to_string()
        ];

        match proc_args::process_args(&args,&mut phrases_file, &mut stop_words_file, &mut context_len, &mut sort_type) {
            Ok(true) => assert!(false, "An error ocurred, help was not called"),
            Ok(false) => assert!(false, "An error ocurred, the program should've failed"),
            Err(s) => assert!(true, "{}", s),
        };
    }

}