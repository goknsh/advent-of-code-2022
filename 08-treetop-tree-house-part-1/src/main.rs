use std::{env, fs};

#[derive(Clone, Copy)]
struct Tree {
    height: i32,
    visible: bool
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut visible = 0;
                let mut trees: Vec<Vec<Tree>> = Vec::new();
                let mut top_maxes: Vec<i32> = vec![-1; 99];

                let mut i = 0;
                for line in input.lines() {
                    let mut j = 0;
                    let mut left_max = -1;
                    trees.insert(i, Vec::new());
                    for letter in line.chars() {
                        let height = letter.to_digit(10).unwrap() as i32;
                        let mut tree = Tree { height, visible: false };

                        if height > top_maxes[j] {
                            tree.visible = true;
                            top_maxes[j] = height;
                        }
                        if height > left_max {
                            tree.visible = true;
                            left_max = height
                        }
                        trees.get_mut(i).unwrap().insert(j, tree);

                        if i == 0 {
                            top_maxes[j] = height
                        }
                        if j == 0 {
                            left_max = height
                        }

                        j += 1;
                    }
                    i += 1;
                }

                let trees_len = trees.len();
                let mut bottom_maxes: Vec<i32> = vec![-1; 99];
                for k in 0..trees.len() {
                    let i = trees.len() - k - 1;
                    let mut right_max = -1;
                    let row_len = trees.get(i).unwrap().len();
                    for l in 0..trees.get(i).unwrap().len() {
                        let j = trees.get(i).unwrap().len() - l - 1;
                        let tree = trees.get_mut(i).unwrap().get_mut(j).unwrap();

                        if tree.height > bottom_maxes[j] {
                            tree.visible = true;
                            bottom_maxes[j] = tree.height
                        }
                        if tree.height > right_max {
                            tree.visible = true;
                            right_max = tree.height
                        }

                        if tree.visible {
                            visible += 1
                        }

                        if i == trees_len {
                            bottom_maxes[j] = tree.height
                        }
                        if j == row_len {
                            right_max = tree.height
                        }
                    }
                }
                println!("Number of trees visible from outside the grid: {}", visible);
            }
            Err(_) => println!("Could not read from provided file.")
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
