use core::str;

use assert_cmd::Command;
use regex::Regex;

#[test]
fn test_no_input() {
    let mut cmd = Command::cargo_bin("rspw").unwrap();
    cmd.assert().failure();
} 

#[test]
fn test_generate_passwd() {
    let pattern = "^[a-zA-Z0-9]";

    for i in 8..=64 {
        test_generate_password_length_pattern(i, pattern);
    }
}

fn test_generate_password_length_pattern(length: u8, pattern: &str) {
    let length_str = &length.to_string();
    
    let mut cmd = Command::cargo_bin("rspw").unwrap();
    let output = cmd.args(&["-l", length_str]).output().unwrap();
    let stdout_str = str::from_utf8(&output.stdout).unwrap().trim();

    let pattern = format!(r"{}{{{}}}$", pattern, length);
    let re = Regex::new(&pattern).unwrap();
    assert_eq!(stdout_str.len(), length.into());
    assert!(re.is_match(stdout_str)); 
}