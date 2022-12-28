use std::{env, fs};
use std::collections::HashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn is_touching(&self, other: &Self) -> bool {
        (self.x == other.x && self.y == other.y) || (self.x == other.x + 1 && self.y == other.y) || (self.x == other.x - 1 && self.y == other.y) || (self.x == other.x && self.y == other.y + 1) || (self.x == other.x && self.y == other.y - 1) || (self.x == other.x + 1 && self.y == other.y + 1) || (self.x == other.x + 1 && self.y == other.y - 1) || (self.x == other.x - 1 && self.y == other.y + 1) || (self.x == other.x - 1 && self.y == other.y - 1)
    }

    fn make_touching(&mut self, head: &Self) {
        if self.x == head.x {
            if head.y < self.y {
                self.y -= 1;
            } else {
                self.y += 1;
            }
        } else if self.y == head.y {
            if head.x < self.x {
                self.x -= 1;
            } else {
                self.x += 1;
            }
        } else if (head.x == self.x + 2 && head.y == self.y + 1) || (head.x == self.x + 1 && head.y == self.y + 2) || (head.x == self.x + 2 && head.y == self.y + 2) {
            self.x += 1;
            self.y += 1
        } else if (head.x == self.x - 2 && head.y == self.y + 1) || (head.x == self.x - 1 && head.y == self.y + 2) || (head.x == self.x - 2 && head.y == self.y + 2) {
            self.x -= 1;
            self.y += 1
        } else if (head.x == self.x - 2 && head.y == self.y - 1) || (head.x == self.x - 1 && head.y == self.y - 2) || (head.x == self.x - 2 && head.y == self.y - 2) {
            self.x -= 1;
            self.y -= 1
        } else if (head.x == self.x + 2 && head.y == self.y - 1) || (head.x == self.x + 1 && head.y == self.y - 2) || (head.x == self.x + 2 && head.y == self.y - 2) {
            self.x += 1;
            self.y -= 1
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut knots = Vec::from([/* H equivalent */ Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }]);
                let mut tail_visited = HashSet::from([*knots.get(9).unwrap()]);

                for line in input.lines() {
                    let movement: Vec<&str> = line.split(" ").collect();
                    let direction = movement.get(0).unwrap();
                    let mut amount = movement.get(1).unwrap().parse::<i32>().unwrap();

                    while amount != 0 {
                        amount -= 1;
                        {
                            let mut head = knots.get_mut(0).unwrap();
                            if *direction == "U" {
                                head.y += 1;
                            } else if *direction == "D" {
                                head.y -= 1
                            } else if *direction == "R" {
                                head.x += 1
                            } else if *direction == "L" {
                                head.x -= 1
                            }
                        }

                        for i in 0..9 {
                            let head = *knots.get(i).unwrap();
                            let tail = knots.get_mut(i+1).unwrap();
                            if !head.is_touching(&tail) {
                                tail.make_touching(&head);
                                if i + 1 == 9 {
                                    tail_visited.insert(*tail);
                                }
                            } else {
                                break;
                            }
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
