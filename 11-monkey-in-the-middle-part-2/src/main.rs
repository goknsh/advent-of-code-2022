use std::{env, fs};
use std::str::FromStr;

struct Monkey<'a> {
    items: Vec<i64>,
    operation: Vec<&'a str>,
    test: i64,
    test_true: usize,
    test_false: usize,
    items_inspected: i64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut monkeys: Vec<Monkey> = Vec::new();
                for mut line in input.lines() {
                    line = line.trim();
                    if line.starts_with("Monkey ") {
                        monkeys.push(Monkey {
                            items: Vec::new(),
                            operation: Vec::new(),
                            test: 0,
                            test_true: 0,
                            test_false: 0,
                            items_inspected: 0
                        })
                    } else if line.starts_with("Starting items: ") {
                        line = line.strip_prefix("Starting items: ").unwrap();
                        monkeys.last_mut().unwrap().items = line.split(", ").map(|s| i64::from_str(s).unwrap()).collect()
                    } else if line.starts_with("Operation: new = ") {
                        line = line.strip_prefix("Operation: new = ").unwrap();
                        monkeys.last_mut().unwrap().operation = line.split(" ").collect()
                    } else if line.starts_with("Test: divisible by ") {
                        line = line.strip_prefix("Test: divisible by ").unwrap();
                        monkeys.last_mut().unwrap().test = line.parse::<i64>().unwrap()
                    } else if line.starts_with("If true: throw to monkey ") {
                        line = line.strip_prefix("If true: throw to monkey ").unwrap();
                        monkeys.last_mut().unwrap().test_true = line.parse::<usize>().unwrap()
                    } else if line.starts_with("If false: throw to monkey ") {
                        line = line.strip_prefix("If false: throw to monkey ").unwrap();
                        monkeys.last_mut().unwrap().test_false = line.parse::<usize>().unwrap()
                    }
                }
                let multiple: i64 = monkeys.iter().map(|m| m.test).product();
                for _ in 0..10000 {
                    let monkeys_len = monkeys.len();
                    for i in 0..monkeys_len {
                        for mut item in monkeys.get(i).unwrap().items.clone() {
                            let operand = monkeys[i].operation[2].parse::<i64>().unwrap_or(item);
                            if monkeys[i].operation[1] == "*" {
                                item *= operand;
                            } else if monkeys[i].operation[1] == "+" {
                                item += operand;
                            } else if monkeys[i].operation[1] == "-" {
                                item -= operand
                            } else if monkeys[i].operation[1] == "/" {
                                item /= operand
                            }

                            item %= multiple;

                            let test_true = monkeys[i].test_true;
                            let test_false = monkeys[i].test_false;

                            if item % monkeys[i].test == 0 {
                                monkeys[test_true].items.push(item)
                            } else {
                                monkeys[test_false].items.push(item)
                            }

                            monkeys[i].items_inspected += 1
                        }
                        monkeys[i].items.clear();
                    }
                }
                let mut max_inspected = Vec::new();
                for monkey in monkeys {
                    max_inspected.insert(0, monkey.items_inspected)
                }
                max_inspected.sort_unstable();
                println!("Level of monkey business after 10000 rounds: {}", max_inspected[max_inspected.len()-1] * max_inspected[max_inspected.len()-2])
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
