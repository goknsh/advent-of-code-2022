use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut max = 0;
                let mut current_value = 0;
                for line in input.lines() {
                    if line.len() == 0 {
                        if current_value > max {
                            max = current_value;
                        }
                        current_value = 0;
                    } else {
                        current_value += line.parse::<i32>().unwrap();
                    }
                }
                println!("Most calories carried by elf: {}", max)
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
