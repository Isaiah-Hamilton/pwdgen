use rand::Rng;
use clap::Parser;

const LOWERCASE_CHARS: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
const UPPERCASE_CHARS: [&str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
const NUMERIC_CHARS: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
const SPECIAL_CHARS: [&str; 27] = ["!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "=", "_", "+", "[", "]", "{", "}", "|", ";", ":", ",", ".", "<", ">", "?", "/"];

/// Simple password generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 32)]
    length: u8,
}

fn main() {
    let args = Args::parse();

    let mut rng = rand::thread_rng();

    let mut password = String::new();
    let pool = 4;

    let mut i = 0;
    while i < args.length {
        if pool == 0 {
            println!("Please provide a char type");
            break;
        }
        let random_pool = rng.gen_range(1..=pool);
        if random_pool == 1 {
            let random_number = rng.gen_range(0..=LOWERCASE_CHARS.len() - 1);
            password += LOWERCASE_CHARS[random_number];
        }
        if random_pool == 2 {
            let random_number = rng.gen_range(0..=UPPERCASE_CHARS.len() - 1);
            password += UPPERCASE_CHARS[random_number];
        }
        if random_pool == 3 {
            let random_number = rng.gen_range(0..=NUMERIC_CHARS.len() - 1);
            password += NUMERIC_CHARS[random_number];
        }
        if random_pool == 4 {
            let random_number = rng.gen_range(0..=SPECIAL_CHARS.len() - 1);
            password += SPECIAL_CHARS[random_number];
        }
        i += 1;
    }

    if !password.is_empty() {
        println!("password: {}", password);
    }
}
