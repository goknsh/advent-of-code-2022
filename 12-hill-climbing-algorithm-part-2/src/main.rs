use std::{env, fs};
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize
}

struct QueueNode {
    distance: i32,
    location: Coordinate
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let total_rows = input.lines().count();
                let total_cols = input.lines().next().unwrap().len();
                let mut heights = vec![vec![0; total_cols]; total_rows];
                let mut start = Coordinate { x: 0, y: 0 };
                let mut end = Coordinate { x: 0, y: 0 };
                for (i, line) in input.lines().enumerate() {
                    for (j, letter) in line.chars().enumerate() {
                        if letter == 'S' {
                            start.x = j;
                            start.y = i;
                            heights[i][j] = ('a' as u8) - 96;
                        } else if letter == 'E' {
                            end.x = j;
                            end.y = i;
                            heights[i][j] = ('z' as u8) - 96;
                        } else {
                            heights[i][j] = (letter as u8) - 96;
                        }
                    }
                }

                let mut queue = vec![QueueNode { distance: 0, location: end }];
                let mut visited = HashSet::from([end]);
                let mut final_distance = 0;

                while !queue.is_empty() {
                    let current = queue.pop().unwrap();
                    let current_height = heights[current.location.y][current.location.x];
                    if current_height == 1 {
                        final_distance = current.distance;
                        break;
                    }

                    let mut visitable_neighbors = Vec::new();
                    if current.location.x > 0 {
                        visitable_neighbors.push(Coordinate { x: current.location.x - 1, y: current.location.y })
                    }
                    if current.location.y > 0 {
                        visitable_neighbors.push(Coordinate { x: current.location.x, y: current.location.y - 1 })
                    }
                    if current.location.x < total_cols - 1 {
                        visitable_neighbors.push(Coordinate { x: current.location.x + 1, y: current.location.y })
                    }
                    if current.location.y < total_rows - 1 {
                        visitable_neighbors.push(Coordinate { x: current.location.x, y: current.location.y + 1 })
                    }

                    let neighbors: Vec<&Coordinate> = visitable_neighbors.iter().filter(|n| heights[n.y][n.x] >= current_height || heights[n.y][n.x] == current_height - 1).collect();
                    for neighbor in neighbors {
                        if visited.insert(*neighbor) {
                            queue.insert(0, QueueNode { distance: current.distance + 1, location: *neighbor })
                        }
                    }
                }

                println!("Fewest steps required to move starting from any square with elevation `a` to the location that should get the best signal: {}", final_distance)
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
