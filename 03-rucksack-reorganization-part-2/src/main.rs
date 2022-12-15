use std::{env, fs};
use std::collections::{HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut sum: i32 = 0;
                let mut line_number = 0;
                let mut letter_appears_in: HashMap<char, i32> = HashMap::with_capacity(52);
                for line in input.lines() {
                    let letters: Vec<char> = line.chars().collect();
                    for letter in letters {
                        if line_number == 0 {
                            match letter_appears_in.get(&letter) {
                                None => {
                                    letter_appears_in.insert(letter, line_number);
                                }
                                _ => {}
                            }
                        } else if line_number == 1 {
                            match letter_appears_in.get_mut(&letter) {
                                Some(last_appears_in) => {
                                    *last_appears_in = line_number;
                                }
                                _ => {}
                            }
                        } else {
                            match letter_appears_in.get(&letter) {
                                Some(last_appears_in) => {
                                    if *last_appears_in == line_number - 1 {
                                        let mut letter_byte = [0; 4];
                                        letter.encode_utf8(&mut letter_byte);
                                        if letter.is_ascii_lowercase() {
                                            sum += i32::from_le_bytes(letter_byte) - 96;
                                        } else if letter.is_ascii_uppercase() {
                                            sum += i32::from_le_bytes(letter_byte) - 38;
                                        }
                                        letter_appears_in.clear();
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    line_number += 1;
                    line_number %= 3;
                }
                println!("Sum of priorities of duplicate items: {}", sum)
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
