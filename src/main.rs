use std::time::Duration;
use std::{fs, thread};
use std::{
    collections::HashMap,
    io::{self, Read},
    time::SystemTime,
};

#[derive(Debug)]
struct State {
    files: HashMap<String, u128>,
}

impl State {
    fn new() -> Self {
        Self {
            files: HashMap::new()
        }
    }
    fn check(&mut self) -> bool {
        let mut any_changed = false;
        for (key, val) in self.files.iter_mut() {
            let last_modified = get_last_modified(&key);
            if val != &last_modified {
                any_changed = true;
                *val = last_modified;
            }
        }
        any_changed
    }
}

fn get_last_modified(file_path: &String) -> u128 {
    fs::metadata(file_path)
        .inspect_err(|e| eprintln!("Failed to read file: {e}"))
        .unwrap()
        .modified()
        .expect("Could not read file modified time")
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get time since unix epoch")
        .as_millis()
}

fn read_input() -> Vec<String> {
    let mut input_buffer: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut input_buffer)
        .expect("Could not read std input");
    let input_as_str =
        String::from_utf8(input_buffer).expect("Could not parse std input as a string");
    input_as_str
        .split("\n")
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .collect()
}

fn main() -> io::Result<()> {
    let mut state = State::new();
    for file_path in read_input() {
        let last_modified = get_last_modified(&file_path);
        state.files.insert(file_path, last_modified);
    }
    loop {
        thread::sleep(Duration::from_millis(100));
        if state.check() {
            // run the desired command
            println!("changed");
        }
    }
}
