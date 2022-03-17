use rspw::{Arguments, generate_password};

#[test]
fn test_arguments_constructor() {
    let test_input = [
        "rspw".to_string(),
        "gen".to_string()
    ];
    let args = Arguments::new(&test_input).unwrap();

    assert_eq!(args.get_query(), test_input[1]);
    assert_eq!(args.get_length(), "8");

    let test_input  = [
        "rspw".to_string(),
        "gen".to_string(),
        "40".to_string()
    ];
    let args = Arguments::new(&test_input).unwrap();

    assert_eq!(args.get_query(), test_input[1]);
    assert_eq!(args.get_length(), "40");
}
#[test]
fn test_generate_password() {
    let mut test_input = [
        "rspw".to_string(),
        "gen".to_string(),
        "40".to_string()
    ];
    let test_args = Arguments::new(&test_input).unwrap();
    assert_eq!(generate_password(test_args).unwrap().len(), 40);

    test_input[2] = "64".to_string();
    let test_args = Arguments::new(&test_input).unwrap();
    assert_eq!(generate_password(test_args).unwrap().len(), 64);

    let test_input  = [
        "rspw".to_string(),
        "gen".to_string(),
    ];
    let test_args = Arguments::new(&test_input).unwrap();
    assert_eq!(generate_password(test_args).unwrap().len(), 8);
}
