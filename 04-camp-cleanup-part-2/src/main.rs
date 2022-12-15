use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut count: i32 = 0;
                for line in input.lines() {
                    let sections: Vec<&str> = line.split(",").collect();
                    let first_elf: Vec<i32> = sections[0].split("-").map(|s| s.parse().unwrap()).collect();
                    let second_elf: Vec<i32> = sections[1].split("-").map(|s| s.parse().unwrap()).collect();

                    if (first_elf[0] <= second_elf[0] && second_elf[0] <= first_elf[1]) || (first_elf[0] <= second_elf[1] && second_elf[1] <= first_elf[1]) || (second_elf[0] <= first_elf[0] && first_elf[0] <= second_elf[1]) || (second_elf[0] <= first_elf[1] && first_elf[1] <= second_elf[1]) {
                        count += 1;
                    }
                }
                println!("Any overlap count: {}", count)
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
