use rspw::{Arguments};

//TODO: All tests need to be rewritten
#[test]
fn test_init() {    
    /*let args = Arguments::init();
    
    assert_eq!(args, ...);*/
}

#[test]
fn test_generate_passwd() {
    let mut test_input = [
        "-l=12".to_string(),
        "-s".to_string()
    ];
    
    let args = Arguments::init();
    assert_eq!(args.generate_passwd().unwrap().len(), 12);
    
    // Max length
    let args = Arguments::init();
    test_input[0] = "-l=64".to_string();
    assert_eq!(args.generate_passwd().unwrap().len(), 64);

    // Test for right error on wrong range
}
