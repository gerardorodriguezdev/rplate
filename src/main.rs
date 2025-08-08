use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, fs};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Target template not defined");
    }

    let target_directory = &args[1];
    let file = File::open(&target_directory);
    let reader = BufReader::new(file?);
    let mut state = State::new();

    for line in reader.lines() {
        let line = line?;

        if line.len() < state.spaces + 1 {
            panic!("Target line is too short");
        }

        for char in line.chars().take(state.spaces) {
            if char != ' ' {
                panic!("Invalid character");
            }
        }

        let is_directory = line.chars().nth(state.spaces).unwrap() == '/';
        if is_directory {
            let directory = line.chars().skip(state.spaces).collect::<String>();
            state.increment_spaces();
            state.append_parent(directory.clone());
            fs::create_dir_all(&state.parent)?;
        } else {
            let file_name = line.chars().skip(state.spaces).collect::<String>();
            let parent = state.parent.clone();
            let path = parent + "/" + file_name.as_str();
            fs::write(path, "")?;
        }
    }

    Ok(())
}

struct State {
    spaces: usize,
    parent: String,
}

impl State {
    fn new() -> Self {
        State {
            spaces: 0,
            parent: String::from(""),
        }
    }

    fn increment_spaces(&mut self) {
        self.spaces += 2;
    }

    fn append_parent(&mut self, new_parent: String) {
        if self.parent.is_empty() {
            let mut other_parent = new_parent.clone();
            other_parent.remove(0);
            self.parent = other_parent;
        } else {
            self.parent += &new_parent;
        }
    }
}
