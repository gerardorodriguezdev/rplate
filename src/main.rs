use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Target template not defined");
    }

    let target_directory = &args[1];
    let file = File::open(&target_directory);
    let reader = BufReader::new(file?);
    let mut spaces = State::new();

    for line in reader.lines() {
        let line = line?;

        if line.len() < spaces.count + 1 {
            panic!("Target line is too short");
        }

        for char in line.chars().take(spaces.count) {
            if char != ' ' {
                panic!("Invalid character");
            }
        }

        let is_directory = line.chars().nth(spaces.count).unwrap() == '/';
        if is_directory {
            println!("Directory");
            spaces.increase_level();
        } else {
            println!("File");
        }
    }

    Ok(())
}

struct State {
    count: usize,
    parent: String,
}

impl State {
    fn new() -> Self {
        State {
            count: 0,
            parent: String::from(""),
        }
    }

    fn increase_level(&mut self) {
        self.count += 2;
    }
}
