use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut sum: i32 = 0;
                for line in input.lines() {
                    let letters: Vec<char> = line.chars().collect();
                    let mut letter_map = HashMap::with_capacity(letters.len() / 2);
                    for i in 0..(letters.len()/2) {
                        letter_map.insert(letters[i], true);
                    }
                    for i in (letters.len()/2)..letters.len() {
                        if letter_map.contains_key(&letters[i]) {
                            let mut letter_byte = [0; 4];
                            letters[i].encode_utf8(&mut letter_byte);
                            if letters[i].is_ascii_lowercase() {
                                sum += i32::from_le_bytes(letter_byte) - 96;
                            } else if letters[i].is_ascii_uppercase() {
                                sum += i32::from_le_bytes(letter_byte) - 38;
                            }
                            break;
                        }
                    }
                }
                println!("Sum of priorities of duplicate items: {}", sum)
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
