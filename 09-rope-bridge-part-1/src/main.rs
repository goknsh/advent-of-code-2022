use std::{env, fs};
use std::collections::{HashSet};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn is_touching(&self, other: &Self) -> bool {
        (self.x == other.x && self.y == other.y) || (self.x == other.x + 1 && self.y == other.y) || (self.x == other.x - 1 && self.y == other.y) || (self.x == other.x && self.y == other.y + 1) || (self.x == other.x && self.y == other.y - 1) || (self.x == other.x + 1 && self.y == other.y + 1) || (self.x == other.x + 1 && self.y == other.y - 1) || (self.x == other.x - 1 && self.y == other.y + 1) || (self.x == other.x - 1 && self.y == other.y - 1)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut head = Coordinate { x: 0, y: 0 };
                let mut tail = Coordinate { x: 0, y: 0 };
                let mut tail_visited = HashSet::from([tail]);
                for line in input.lines() {
                    let movement: Vec<&str> = line.split(" ").collect();
                    let direction = movement.get(0).unwrap();
                    let mut amount = movement.get(1).unwrap().parse::<i32>().unwrap();

                    while amount != 0 {
                        amount -= 1;
                        if *direction == "U" {
                            head.y += 1;
                        } else if *direction == "D" {
                            head.y -= 1
                        } else if *direction == "R" {
                            head.x += 1
                        } else if *direction == "L" {
                            head.x -= 1
                        }

                        if !head.is_touching(&tail) {
                            if tail.x == head.x {
                                if head.y < tail.y {
                                    tail.y -= 1;
                                } else {
                                    tail.y += 1;
                                }
                            } else if tail.y == head.y {
                                if head.x < tail.x {
                                    tail.x -= 1;
                                } else {
                                    tail.x += 1;
                                }
                            } else if (head.x == tail.x + 2 && head.y == tail.y + 1) || (head.x == tail.x + 1 && head.y == tail.y + 2) || (head.x == tail.x + 2 && head.y == tail.y + 2) {
                                tail.x += 1;
                                tail.y += 1
                            } else if (head.x == tail.x - 2 && head.y == tail.y + 1) || (head.x == tail.x - 1 && head.y == tail.y + 2) || (head.x == tail.x - 2 && head.y == tail.y + 2) {
                                tail.x -= 1;
                                tail.y += 1
                            } else if (head.x == tail.x - 2 && head.y == tail.y - 1) || (head.x == tail.x - 1 && head.y == tail.y - 2) || (head.x == tail.x - 2 && head.y == tail.y - 2) {
                                tail.x -= 1;
                                tail.y -= 1
                            } else if (head.x == tail.x + 2 && head.y == tail.y - 1) || (head.x == tail.x + 1 && head.y == tail.y - 2) || (head.x == tail.x + 2 && head.y == tail.y - 2) {
                                tail.x += 1;
                                tail.y -= 1
                            }
                            tail_visited.insert(tail);
                        }
                    }
                }
                println!("Positions tail of the rope visit at least once: {}", tail_visited.len())
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
