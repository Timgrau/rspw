use rspw::Arguments;
use std::process;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

/*
Name: rust-cli-password-generator
command : rspw
length: 8-64
usage: rspw -l 16 -s -c 

TODO:
   1. Write password into a selected file :: rspw -l 8 --file=example.txt
   2. Put password on the clipboard for just 30 s
   2. Rewrite tests
 */

fn main() {
    let input = Arguments::init();
    
    let password = input.generate_passwd().unwrap_or_else(|err| {
        println!("Password cant be generated: {}", err);
        process::exit(1);
    });

    if input.clipboard {
        clip_passwd(password);
    } else {
        println!("{}", password);
    }
}

fn clip_passwd(password: String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password.to_owned()).unwrap();
    ctx.get_contents().unwrap();
    /* 2. TODO here
    ctx.clear().unwrap();
    ctx.get_contents().unwrap();*/
}
