use rspw::{Arguments, generate_password};

#[test]
fn test_arguments_constructor() {
    let mut test_input = [
        "gen".to_string(),
        "".to_string(),
    ];
    
    // Default length -> 8
    let args = Arguments::new(&test_input[0], &test_input[1]);
     
    assert_eq!(args.get_query(), test_input[0]);
    assert_eq!(args.get_length(), "8");

    // Any length 
    test_input[1] = "40".to_string();
    let args = Arguments::new(&test_input[0], &test_input[1]);
    
    assert_eq!(args.get_query(), test_input[0]);
    assert_eq!(args.get_length(), "40");
}

#[test]
fn test_generate_password() {
    let mut test_input = [
        "gen".to_string(),
        "40".to_string()
    ];
    
    let test_args = Arguments::new(test_input[0], test_input[1]);
    assert_eq!(generate_password(test_args).unwrap().len(), 40);
    
    // Max length
    test_input[1] = "64".to_string();
    let test_args = Arguments::new(test_input[0], test_input[1]);
    assert_eq!(generate_password(test_args).unwrap().len(), 64);

    // Default length
    test_input[1] = "".to_string();
    let test_args = Arguments::new(test_input[0], test_input[1]);
    assert_eq!(generate_password(test_args).unwrap().len(), 8);
}
