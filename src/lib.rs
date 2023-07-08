use clap::Parser;
use rand::distributions::{Alphanumeric, Distribution};
use rand::rngs::OsRng;
use rand::Rng;
use std::ffi::OsString;

pub struct Special;
impl Distribution<u8> for Special {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789\
                                 !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~\
                                 \x20";

        // Using uniform distribution
        return CHARSET[rng.gen_range(0..CHARSET.len())];
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, arg_required_else_help = true)]
pub struct Arguments {
    /// length of the password [6-64]
    #[arg(short, value_parser = clap::value_parser!(u8).range(6..65))]
    length: u8,

    /// special characters
    #[arg(short, long)]
    special: bool,

    /// Copy password onto the clipboard
    #[arg(short)]
    pub clipboard: bool,
}

impl Arguments {
    /**
      Arguments init functions are just wrapper for `parse()` and
      `parse_from()`, so `clap::Parser` is just used in this file.
    */
    pub fn init() -> Self {
        Arguments::parse()
    }

    pub fn init_daemon<I, T>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Arguments::parse_from(itr)
    }
    /**
      Logic/Function to generate the password, considers special
      characters if `-s | --special` flag was specified.
    */
    pub fn generate_passwd(&self) -> Result<String, &'static str> {
        if self.length < 6 || self.length > 64 {
            return Err("Second Argument needs to be a number in range [6, 64] e.g. 'rspw -l 16'.");
        }

        if !self.special {
            Ok(OsRng
                .sample_iter(&Alphanumeric)
                .take(self.length as usize)
                .map(char::from)
                .collect())
        } else {
            Ok(OsRng
                .sample_iter(&Special)
                .take(self.length as usize)
                .map(char::from)
                .collect())
        }
    }
}
