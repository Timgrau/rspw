use arboard::Clipboard;
use itertools::Itertools;
use rspw::Arguments;
use std::{
    env::{self, args, args_os},
    ffi::OsString,
    process, thread, time,
};

/**
* Name: rust-cli-password-generator
* command : rspw
* length: 8-64
* usage: rspw -l 16 -s -c

* TODO:
*  1. Implement/Test further OS (Win, BSD, ...)
*  2. Rewrite tests
*  3. Check security aspects of this rspw daemon
*/

/* Daemonize arg */
const DAEMONIZE: &str = "__rspw_daemon";
/* Time to clip passwd on the clipboard in sec */
const SLEEP_TIME: u64 = 30;

fn main() {
    let input = check_rspw_daemon();

    if input.clipboard {
        spawn_rspw_daemon();
    } else {
        let password = input.generate_passwd().unwrap_or_else(|err| {
            println!("Password cant be generated: {}", err);
            process::exit(1);
        });
        if args().nth(args_os().len() - 1).as_deref() == Some(DAEMONIZE) {
            let _ = rspw_clipboard(password);
        } else {
            println!("{}", password);
        }
    }
}
/**
  Needet to check if the clipboard daemon is running,
  if not we start processing the input arguments.
*/
fn check_rspw_daemon() -> Arguments {
    if args().nth(args_os().len() - 1).as_deref() == Some(DAEMONIZE) {
        Arguments::init_daemon(args_os().dropping_back(1))
    } else {
        Arguments::init()
    }
}
/**
   Put the password on the clipboard and clear the clipboard after
   the proc has slept for `SLEEP_TIME`. Seems like this needs to return
   `Ok(())` in order to function propertly.
*/
fn rspw_clipboard(password: String) -> Result<(), ()> {
    let mut clipboard = Clipboard::new().unwrap_or_else(|err| {
        println!("Could not create a clipboard instance: {}", err.to_string());
        process::exit(1);
    });
    clipboard.set_text(password).unwrap_or_else(|err| {
        println!("Could not set text to the clipboard: {}", err.to_string());
        process::exit(1);
    });
    thread::sleep(time::Duration::from_secs(SLEEP_TIME));

    clipboard.clear().unwrap_or_else(|err| {
        println!("Could not clear the clipboard: {}", err.to_string());
        process::exit(1);
    });

    Ok(())
}

/**
  Spawn a daemon that runs `rspw` and passes the `filtered_args()`, the
  `DAEMONIZE` arg of this proc will be checked in `check_rspw_daemon()`
  and `main()`.
*/
fn spawn_rspw_daemon() {
    // 1. TODO: Implement further OS
    println!(
        "Password will be copied on the clipboard, clears in {} seconds.",
        SLEEP_TIME
    );
    if cfg!(target_os = "linux") {
        process::Command::new(env::current_exe().unwrap_or_else(|err| {
            println!("Could not find current path: {}", err.to_string());
            process::exit(1);
        }))
        .args(filtered_args())
        .arg(DAEMONIZE)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .current_dir("/")
        .spawn()
        .unwrap_or_else(|err| {
            println!("Could not spawn child process: {}", err.to_string());
            println!("Cannot add password to the clipboard!");
            process::exit(1);
        });
    }
}
/**
  Filter `"-c"` from args, so we can enter the `else` statement in `main()`, also reomve
  `current_exe()` (first argument), because we explicitly set it in `spawn_rspw_daemon()`.

  For that we first need to make a vec out of the Iter and make it an Iter again.
  This function will just be used in `spawn_rspw_daemon()`.
*/
fn filtered_args() -> impl Iterator<Item = OsString> {
    let mut filt_vec: Vec<OsString> = args_os().filter(|x| x != "-c").collect();
    /* `current_exe()` should always be the first element */
    let _ = filt_vec.remove(0);

    filt_vec.into_iter()
}
