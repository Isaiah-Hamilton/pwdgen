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
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let mut password = String::new();
    let char_sets = [LOWERCASE_CHARS, UPPERCASE_CHARS, NUMERIC_CHARS, SPECIAL_CHARS];
    
    for _ in 0..args.length {
        let random_pool = rng.gen_range(0..char_sets.len());
        let char_set = &char_sets[random_pool];
        let random_char = rng.gen_range(0..char_set.len());
        password.push(char_set.chars().nth(random_char).unwrap());
    }

    println!("password: {}", password);
}
