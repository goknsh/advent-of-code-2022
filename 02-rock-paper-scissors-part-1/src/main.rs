use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut score = 0;
                for line in input.lines() {
                    let game: Vec<&str> = line.split(" ").collect();

                    let mut shape_score = 1;
                    if game[1] == "Y" {
                        shape_score = 2;
                    } else if game[1] == "Z" {
                        shape_score = 3;
                    }

                    let mut outcome_score = 0;
                    if (game[0] == "A" && game[1] == "X") || (game[0] == "B" && game[1] == "Y") || (game[0] == "C" && game[1] == "Z") {
                        outcome_score = 3;
                    }
                    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
                    else if (game[0] == "A" && game[1] == "Y") || (game[0] == "B" && game[1] == "Z") || (game[0] == "C" && game[1] == "X") {
                        outcome_score = 6;
                    }

                    println!("{} vs {} => {} += {} + {}", game[0], game[1], score, shape_score, outcome_score);
                    score += shape_score + outcome_score;
                }
                println!("Total score according to strategy guide: {}", score)
            }
            Err(_) => println!("Could not read from provided file.")
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
