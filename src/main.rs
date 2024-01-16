use rand::Rng;
use clap::Parser;

const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMERIC_CHARS: &str = "0123456789";
const SPECIAL_CHARS: &str = "!@#$%^&*()-=_+[]{}|;:,.<>?/";

/// Simple password generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 16)]
    length: u8,

    /// Add lowercase letters to the password [default: true]
    #[arg(long, default_value_t = true)]
    lowercase: bool,

    /// Add uppercase letters to the password [default: true]
    #[arg(long, default_value_t = true)]
    uppercase: bool,

    /// Add special characters to the password [default: true]
    #[arg(long, default_value_t = true)]
    special: bool,

    /// Add numbers to the password [default: true]
    #[arg(long, default_value_t = true)]
    numbers: bool
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let mut password = String::new();
    let mut char_sets = Vec::new();

    if args.lowercase {
        char_sets.push(LOWERCASE_CHARS);
    }

    if args.uppercase {
        char_sets.push(UPPERCASE_CHARS);
    }

    if args.numbers {
        char_sets.push(NUMERIC_CHARS);
    }

    if args.special {
        char_sets.push(SPECIAL_CHARS);
    }

    for _ in 0..args.length {
        let random_pool = rng.gen_range(0..char_sets.len());
        let char_set = &char_sets[random_pool];
        let random_char = rng.gen_range(0..char_set.len());
        password.push(char_set.chars().nth(random_char).unwrap());
    }

    println!("password: {}", password);
}
