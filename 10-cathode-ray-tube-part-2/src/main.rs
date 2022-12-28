use std::{env, fs};

struct CPU {
    clock: usize,
    register_x: i32,
    display: Vec<bool>
}

impl CPU {
    fn execute_noop(&mut self) {
        self.clock += 1;
        if self.register_x as usize <= self.clock % 40 && self.clock % 40 <= self.register_x as usize + 2 {
            self.display[self.clock - 1] = true
        } else {
            self.display[self.clock - 1] = false
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
                let mut cpu = CPU { clock: 0, register_x: 1, display: vec![false; 240] };

                for line in input.lines() {
                    if line == "noop" {
                        cpu.execute_noop()
                    } else {
                        let cmd: Vec<&str> = line.split(" ").collect();
                        let amount = cmd.get(1).unwrap().parse::<i32>().unwrap();
                        cpu.execute_addx(amount)
                    }
                }

                for i in 0..cpu.display.len() {
                    if *cpu.display.get(i).unwrap() {
                        print!("#")
                    } else {
                        print!(".")
                    }
                    if (i + 1) % 40 == 0 {
                        println!()
                    }
                }
            }
            Err(_) => println!("Could not read from provided file."),
        }
    } else {
        println!("Must provide input.txt file as an argument.")
    }
}
