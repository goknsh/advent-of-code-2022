use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut lines_iter = input.lines().into_iter();
                lines_iter.next();

                let mut directory_sizes = HashMap::new();
                let mut current_path = Vec::new();

                while let Some(line) = lines_iter.next() {
                    if line.starts_with("$ ls") || line.starts_with("dir") {
                        continue;
                    } else if line.starts_with("$ cd ..") {
                        current_path.pop();
                    } else if line.starts_with("$ cd") {
                        let cmd: Vec<&str> = line.split(" ").collect();
                        current_path.push(*cmd.last().unwrap());
                    } else {
                        let cmd: Vec<&str> = line.split(" ").collect();
                        let size = cmd.get(0).unwrap().parse::<u32>().unwrap();
                        for i in 0..current_path.len() {
                            *directory_sizes.entry(current_path[..=i].join("/")).or_insert(0) += size;
                        }
                    }
                }

                println!("Directories with a total size of at most 100000: {}", directory_sizes.into_values().filter(|s| *s <= 100000).sum::<u32>())
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
