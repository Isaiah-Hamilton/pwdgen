use rand::Rng;

const LOWERCASE_CHARS: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
const UPPERCASE_CHARS: [&str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
const NUMERIC_CHARS: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
const SPECIAL_CHARS: [&str; 27] = ["!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "=", "_", "+", "[", "]", "{", "}", "|", ";", ":", ",", ".", "<", ">", "?", "/"];

fn main() {
    let mut rng = rand::thread_rng();

    let mut password = String::new();

    let mut i = 1;
    while i < 32 {
    let random_number = rng.gen_range(0..=LOWERCASE_CHARS.len());
        password += LOWERCASE_CHARS[random_number];
        i += 1;
    }

    println!("password: {}", password);
}
