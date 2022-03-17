use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Debug)]
pub struct Arguments {
    // should be 'gen'
    query: String,
    // length of the password
    length: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() <= 1 {
            return Err("Too few arguments, you must pass at least one argument.");
        } else if args.len() > 3 {
            return Err("Too many arguments, you can pass a maximum of two arguments.");
        }

        let query = args[1].clone();
        let length: String;

        if args.len() != 3 {
            // default
            length = String::from("8");
        } else {
            length = args[2].clone();
        }

        Ok(Arguments { query, length })
    }
    // getter for testing
    pub fn get_query(&self) -> &str {
        self.query.as_str()
    }
    pub fn get_length(&self) -> &str {
        self.length.as_str()
    }
}

pub fn generate_password(args: Arguments) -> Result<String, &'static str> {
    if args.query != "gen" {
        return Err("First argument needs to be 'gen' to generate a new password.");
    }
    let length: usize = args.length.parse().unwrap_or(0);

    if length < 6 || length > 64 {
        return Err("Second Argument needs to be a number in range [6, 64] e.g. 'gen 16'.");
    }

    Ok(
        thread_rng().sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from).collect()
    )
}