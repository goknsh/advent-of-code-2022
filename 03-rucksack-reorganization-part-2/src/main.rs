use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut picked_map: HashMap<i32, bool> = HashMap::new();
                let mut letter_map: HashMap<char, [i32;]> = HashMap::with_capacity(52);
                let mut current_line = 0;
                for line in input.lines() {
                    for letter in line.chars().collect() {
                        match letter_map.get(letter) {
                            Some(x) => {
                                let mut picks_found = [-1; 2];
                                let mut current_pick = 0;
                                for i in x {
                                    if i != current_line && !picked_map.get(i).unwrap() {
                                        picks_found[current_pick] = *i;
                                        current_pick += 1;
                                        if current_pick == 2 {
                                            break;
                                        }
                                    }
                                }
                                if current_pick == 2 {
                                    picked_map.insert(current_line, true);
                                    for i in picks_found {
                                        picked_map.insert(i, true);
                                    }
                                    break;
                                }
                            }
                            None => {
                                letter_map.insert(*letter, [current_line]);
                            },
                        }
                    }
                }
                let mut sum = 0;
                for key in sum_map.keys() {
                    if sum_map.get(key).unwrap() % 3 == 0 {
                        let mut letter_byte = [0; 4];
                        key.encode_utf8(&mut letter_byte);
                        if key.is_ascii_lowercase() {
                            sum += (i32::from_le_bytes(letter_byte) - 96) * sum_map.get(key).unwrap() / 3;
                        } else if key.is_ascii_uppercase() {
                            sum += (i32::from_le_bytes(letter_byte) - 38) * sum_map.get(key).unwrap() / 3;
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
