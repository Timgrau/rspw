use rspw::{Arguments};
use std::process;

/*
Name: rust-cli-password-generator
command : rspw
length: 8-64
usage: rspw -l 16 -s

TODO:
   1. Write password into a selected file :: rspw -l 8 example.txt
   2. Put password on the clipboard :: rspw -c -l=8 -s
   3. Rewrite tests
 */

fn main() {
    let input = Arguments::init();
    
    let password = input.generate_passwd().unwrap_or_else(|err| {
        println!("Password cant be generated: {}", err);
        process::exit(1);
    });

    println!("{}", password);
}
