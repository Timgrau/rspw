use rspw::Arguments;
use std::{process, env::{self}, thread, time};
use arboard::{Clipboard};
use itertools::Itertools;

/**
 * Name: rust-cli-password-generator
 * command : rspw
 * length: 8-64
 * usage: rspw -l 16 -s -c 

 * TODO:
 *  1. Get the ppid of the daemon and compare with a passed pid.
 *  2. Implement/Test further OS (Win, BSD, ...)
 *  3. Rewrite tests
 *  4. Security aspect of this clipboard daemon?
 */

/* Daemonize arg */
const DAEMONIZE: &str = "__rspw_daemon";
/* Time to clip passwd on the clipboard in sec */
const SLEEP_TIME: u64 = 5; 

fn main() {
    let input = check_rspw_daemon();
    //println!("{:?}", input);

    if input.clipboard {
        spawn_rspw_daemon();
    } else {
        let password = input.generate_passwd().unwrap_or_else(|err| {
            println!("Password cant be generated: {}", err);
            process::exit(1);
        });
        println!("{}", password);
    }
}
/**
   Needet to check if the clipboard daemon is running,
   if not we start processing the input arguments.
 */
fn check_rspw_daemon() -> Arguments {
    // 1. TODO: Compare ppid with `arg(process::id().to_string())`
    if env::args().nth(env::args_os().len() - 1).as_deref() == Some(DAEMONIZE) {
        println!("here {:?}", env::args_os().dropping_back(
            env::args().position(|x| x.eq("-c")).unwrap()
        ));
        println!("here {:?}", env::args_os().dropping(
            env::args_os().position(|x| x.eq("-c")).unwrap() + 1
        ));
        Arguments::init_daemon(env::args_os().dropping_back(1))
    } else {
        Arguments::init()
    }
}
/**
   REWORK: Put password on the clipboard.
 */
fn clipboard_daemon() -> Result<(), Arguments> {
    if env::args().nth(1).as_deref() == Some(DAEMONIZE) {
        let mut clipboard = Clipboard::new().unwrap_or_else(|err| {
            println!("Could not create a clipboard instance: {}", err.to_string());
            process::exit(1);
        });
        for arg in env::args() {
            clipboard.set_text(arg).unwrap_or_else(|err| {
                println!("Could not set text to the clipboard: {}", err.to_string());
                process::exit(1);
            });
            thread::sleep(time::Duration::from_secs(SLEEP_TIME));
        }
        clipboard.clear().unwrap_or_else(|err| {
            println!("Could not clear the clipboard: {}", err.to_string());
            process::exit(1);
        });

        Ok(())
    } else {
        /* May be theres a clearer way to do this */
        Err(Arguments::init())
    }
}

/**
   Spawn a daemon that holds the passwd, this 
   proc will be checked in `clipboard_daemon()`. 
 */
fn spawn_rspw_daemon() {
    // REWORK: -c need to be removed from env::args_os() 
    println!("here {:?}", env::args_os().dropping_back(
        env::args().position(|x| x.eq("-c")).unwrap()
    ));
    println!("here {:?}", env::args_os().dropping(
        env::args_os().position(|x| x.eq("-c")).unwrap() + 1
    ));
    // 2. TODO: Implement further OS
    if cfg!(target_os = "linux") {
        process::Command::new(env::current_exe()
        .expect("Could not find current path"))
            .args(env::args_os().into_iter().filter(|x| x.ne("-c")))
            .arg(DAEMONIZE)
            // .arg(process::id().to_string())
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir("/")
            .spawn().unwrap_or_else(|err| {
                println!("Could not spawn child process: {}", err.to_string());
                println!("Cannot add password to the clipboard!");
                process::exit(1);
            });
    }
}