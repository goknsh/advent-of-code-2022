use std::{cmp, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut lines = input.lines();
                let mut stacks: Vec<Vec<&str>> = Vec::new();

                let mut first_line = lines.next().unwrap();
                while !first_line.is_empty() {
                    let (stack, rest) = first_line.split_at(cmp::min(4, first_line.len()));
                    if stack.trim().len() != 0 {
                        stacks.push(vec![stack.trim_matches(|c| c == '[' || c == ']' || c == ' ')])
                    } else {
                        stacks.push(Vec::new());
                    }
                    first_line = rest;
                }

                let mut lines_iter = lines.into_iter();

                while let Some(mut line) = lines_iter.next() {
                    if line.starts_with(" 1") {
                        break;
                    }
                    let mut current_stack = 0;
                    while !line.is_empty() {
                        let (stack, rest) = line.split_at(cmp::min(4, line.len()));
                        if stack.trim().len() != 0 {
                            stacks.get_mut(current_stack).unwrap().insert(0, stack.trim_matches(|c| c == '[' || c == ']' || c == ' '));
                        }
                        line = rest;

                        current_stack += 1;
                    }
                }

                lines_iter.next();

                while let Some(line) = lines_iter.next() {
                    let action: Vec<&str> = line.split(' ').collect();
                    let mut number_moved = 0;
                    let mut move_count = action.get(1).unwrap().parse::<usize>().unwrap();
                    let move_from = action.get(3).unwrap().parse::<usize>().unwrap();
                    let move_to = action.get(5).unwrap().parse::<usize>().unwrap();
                    while move_count != 0 {
                        let moving_item = stacks.get_mut(move_from - 1).unwrap().pop().unwrap();
                        let move_to_stack = stacks.get_mut(move_to - 1).unwrap();
                        move_to_stack.insert(move_to_stack.len() - number_moved, moving_item);
                        move_count -= 1;
                        number_moved += 1;
                    }
                }

                print!("Crates on top of each stack: ");
                for stack in stacks {
                    print!("{}", stack.last().unwrap());
                }
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
