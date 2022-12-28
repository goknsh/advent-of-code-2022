use std::{env, fs};

struct CPU {
    clock: i32,
    register_x: i32,
    signal_strengths_sum: i32
}

impl CPU {
    fn execute_noop(&mut self) {
        self.clock += 1;
        if [20, 60, 100, 140, 180, 220].contains(&self.clock) {
            self.signal_strengths_sum += self.clock * self.register_x
        }
    }

    fn execute_addx(&mut self, increase_by: i32) {
        self.execute_noop();
        self.execute_noop();
        self.register_x += increase_by
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(input) => {
                let mut cpu = CPU { clock: 0, register_x: 1, signal_strengths_sum: 0 };

                for line in input.lines() {
                    if line == "noop" {
                        cpu.execute_noop()
                    } else {
                        let cmd: Vec<&str> = line.split(" ").collect();
                        let amount = cmd.get(1).unwrap().parse::<i32>().unwrap();
                        cpu.execute_addx(amount)
                    }
                }
                println!("Sum of six signal strengths: {}", cpu.signal_strengths_sum)
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
