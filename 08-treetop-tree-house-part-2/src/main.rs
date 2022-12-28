use std::{env, fs};

#[derive(Clone, Copy)]
struct Tree {
    height: i32,
    left_visibility: i32,
    top_visibility: i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut max_score = 0;
                let mut trees: Vec<Vec<Tree>> = Vec::new();

                let mut i = 0;
                for line in input.lines() {
                    let mut j = 0;
                    trees.insert(i, Vec::new());
                    for letter in line.chars() {
                        let height = letter.to_digit(10).unwrap() as i32;
                        let mut tree = Tree {
                            height,
                            left_visibility: 0,
                            top_visibility: 0
                        };

                        for top_i in (0..i).rev() {
                            tree.top_visibility += 1;
                            if trees.get(top_i).unwrap().get(j).unwrap().height >= tree.height {
                                break;
                            }
                        }
                        for left_i in (0..j).rev() {
                            tree.left_visibility += 1;
                            if trees.get(i).unwrap().get(left_i).unwrap().height >= tree.height {
                                break;
                            }
                        }

                        trees.get_mut(i).unwrap().insert(j, tree);

                        j += 1;
                    }
                    i += 1;
                }

                let trees_len = trees.len();
                for k in 0..trees.len() {
                    let i = trees_len - k - 1;
                    let row_len = trees.get(i).unwrap().len();
                    for l in 0..trees.get(i).unwrap().len() {
                        let j = row_len - l - 1;
                        let tree = trees.get(i).unwrap().get(j).unwrap();
                        let mut bottom_visibility = 0;
                        let mut right_visibility = 0;

                        for bottom_i in i+1..trees_len {
                            bottom_visibility += 1;
                            if trees.get(bottom_i).unwrap().get(j).unwrap().height >= tree.height {
                                break;
                            }
                        }
                        for right_i in j+1..row_len {
                            right_visibility += 1;
                            if trees.get(i).unwrap().get(right_i).unwrap().height >= tree.height {
                                break;
                            }
                        }

                        let visibility_score = tree.left_visibility * right_visibility * tree.top_visibility * bottom_visibility;
                        if visibility_score > max_score {
                            max_score = visibility_score
                        }
                    }
                }
                println!("Highest scenic score possible for any tree: {}", max_score);
            }
            Err(_) => println!("Could not read from provided file.")
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
