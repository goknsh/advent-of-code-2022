use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut position: i32 = 0;
                let mut last_distinct: Vec<char> = Vec::with_capacity(3);
                for line in input.lines() {
                    let letters: Vec<char> = line.chars().collect();

                    for letter in letters {
                        position += 1;
                        if last_distinct.len() == 3 && !last_distinct.contains(&letter) {
                            break;
                        } else if last_distinct.contains(&letter) {
                            while last_distinct.get(0) != Some(&letter) {
                                last_distinct.remove(0);
                            }
                            last_distinct.remove(0);
                        }
                        last_distinct.push(letter);

                    }
                }
                println!("Start-of-packet marker detected: {}", position)
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
