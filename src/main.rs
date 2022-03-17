use std::env;
use rspw::{Arguments, generate_password};
use std::process;

/*
Name: rust-cli-password-generator

command : rspw
query: gen
length: 8-64
usage: rspw gen 8

TODO:
 1. Generate a password containing special characters :: rspw gen -s 16
 2. Write password into a selected file :: rspw gen 8 example.txt
 */

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = Arguments::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let password = generate_password(input).unwrap_or_else(|err| {
        println!("Password cant be generated: {}", err);
        process::exit(1);
    });

    println!("{}", password);
}