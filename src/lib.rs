use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, arg_required_else_help = true)]
pub struct Arguments {
    /// length of the password
    #[arg(short)]
    length: u8,
}

impl Arguments {
    pub fn init() -> Self {
        Arguments::parse()
    }
}

pub fn generate_password(args: Arguments) -> Result<String, &'static str> {
    if args.length < 6 || args.length > 64 {
        return Err("Second Argument needs to be a number in range [6, 64] e.g. 'rspw -l 16'.");
    }
    
    Ok(
        thread_rng().sample_iter(&Alphanumeric)
            .take(args.length as usize)
            .map(char::from).collect()
    )
}
