use rspw::{generate_password, Arguments};
use std::process;

/*
Name: rust-cli-password-generator

command : rspw
length: 8-64
usage: rspw -l 16

TODO:
 1. Generate a password containing special characters :: rspw gen -s 16
 2. Write password into a selected file :: rspw -l 8 example.txt
 */

fn main() {
    let input = Arguments::init();

    let password = generate_password(input).unwrap_or_else(|err| {
        println!("Password cant be generated: {}", err);
        process::exit(1);
    });

    println!("{}", password);
}
