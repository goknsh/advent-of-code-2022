use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut max = [0, 0, 0];
                let mut current_value = 0;
                for line in input.lines() {
                    if line.len() == 0 {
                        for i in 0..max.len() {
                            if current_value > max[i] {
                                for j in (i..max.len()).rev() {
                                    if i == j {
                                        max[i] = current_value;
                                    } else {
                                        max[j] = max[j-1];
                                    }
                                }
                                break;
                            }
                        }
                        current_value = 0;
                    } else {
                        current_value += line.parse::<i32>().unwrap();
                    }
                }
                let mut total = 0;
                print!("Top three: ");
                for i in max {
                    total += i;
                    print!("{} ", i)
                }
                print!("\n");
                println!("Calories carried in total: {}", total);
            }
            Err(_) => println!("Could not read from provided file.")
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
