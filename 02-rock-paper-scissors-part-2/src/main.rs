use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut score = 0;
                for line in input.lines() {
                    let game: Vec<&str> = line.split(" ").collect();

                    let mut outcome_score = 0;
                    if game[1] == "Y" {
                        outcome_score = 3;
                    } else if game[1] == "Z" {
                        outcome_score = 6;
                    }

                    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
                    let mut shape_score = 1;
                    if game[0] == "A" {
                        if outcome_score == 0 {
                            shape_score = 3;
                        } else if outcome_score == 3 {
                            shape_score = 1
                        } else if outcome_score == 6 {
                            shape_score = 2;
                        }
                    } else if game[0] == "B" {
                        if outcome_score == 0 {
                            shape_score = 1;
                        } else if outcome_score == 3 {
                            shape_score = 2;
                        } else if outcome_score == 6 {
                            shape_score = 3;
                        }
                    } else if game[0] == "C" {
                        if outcome_score == 0 {
                            shape_score = 2;
                        } else if outcome_score == 3 {
                            shape_score = 3;
                        } else if outcome_score == 6 {
                            shape_score = 1;
                        }
                    }

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
