use clap::Parser;
use rand::distr::{Alphanumeric, Distribution};
use rand::rngs::{StdRng, SysRng};
use rand::{Rng, RngExt, SeedableRng};
use std::ffi::OsString;

pub struct Special;
impl Distribution<u8> for Special {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789\
                                 !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

        // Using uniform distribution
        return CHARSET[rng.random_range(0..CHARSET.len())]
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
      Arguments init functions is just a wrapper for `parse()` and
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

        let mut rng = StdRng::try_from_rng(&mut SysRng)
            .expect("failed to seed StdRng from the operating system");

        if !self.special {
            Ok(Alphanumeric
                .sample_iter(&mut rng)
                .take(self.length as usize)
                .map(char::from)
                .collect())
        } else {
            Ok(Special
                .sample_iter(&mut rng)
                .take(self.length as usize)
                .map(char::from)
                .collect())
        }
    }
}
