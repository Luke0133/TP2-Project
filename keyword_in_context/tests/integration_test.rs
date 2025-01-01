use keyword_in_context::keyword_in_context::execute;

#[test]
pub fn test_help(){
    let args = vec!["target\\debug\\keyword_in_context.exe".to_string(),"args".to_string(),"-h".to_string()];

    match execute(args) {
        Ok(true) => assert!(true, "All in order"), 
        Ok(false) => assert!(false, "An error ocurred, the program called for help"),
        Err(s) => assert!(false, "{}", s),
    };
}

#[test]
pub fn test_no_args(){
    let args = vec!["target\\debug\\keyword_in_context.exe".to_string()];

    match execute(args) {
        Ok(true) => assert!(false, "An error ocurred, the program didn't call for help"),
        Ok(false) => assert!(true, "All in order"), 
        Err(s) => assert!(false, "{}", s),
    };
}

#[test]
pub fn test_args_invalid(){
    let args = vec![
            "target\\debug\\keyword_in_context.exe".to_string(),
            "args".to_string(),
            "-p".to_string(),      // There should be an error here, since there's no string following '-p' 
            "-l".to_string(),
            "4".to_string()
        ];

    match execute(args) {
        Ok(true) => assert!(false, "An error ocurred, help was not called"),
        Ok(false) => assert!(false, "An error ocurred, the program should've failed"),
        Err(s) => assert!(true, "{}", s),
    };
}

#[test]
pub fn test_args_invalid_file(){
    let args = vec![
            "target\\debug\\keyword_in_context.exe".to_string(),
            "args".to_string(),
            "-p".to_string(),
            "empty.txt".to_string()
        ];

    match execute(args) {
        Ok(true) => assert!(false, "An error ocurred, help was not called"),
        Ok(false) => assert!(false, "An error ocurred, the program should've failed"),
        Err(s) => assert!(true, "{}", s),
    };
}