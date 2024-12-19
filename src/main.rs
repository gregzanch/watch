use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{
    collections::HashMap,
    io::{self, Read},
    time::SystemTime,
};
use std::{env, fs, process, thread};

#[derive(Debug)]
struct State {
    files: HashMap<String, u128>,
    command: String,
    command_args: Vec<String>,
}

impl State {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
            command: String::new(),
            command_args: Vec::new(),
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

fn parse_args(state: &mut State) {
    let args: Vec<String> = env::args().collect();
    let help_text = include_str!("../help.txt");
    match args.len() {
        1 => {
            println!("No command found\n");
            println!("{}", help_text);
            process::exit(1);
        }
        2 => {
            let split_command_string: Vec<String> = args[1]
                .clone()
                .split(" ")
                .map(str::to_string)
                .filter(|x| !x.is_empty())
                .collect();
            let command = split_command_string.first().expect("Could not parse command");
            let command_args = split_command_string.split_at(1).1;
            state.command = command.to_owned();
            state.command_args = Vec::from(command_args);
        }
        _ => {
            println!("Invalid usage...\n");
            println!("{}", help_text);
            process::exit(1);
        }
    }
}

fn main() -> io::Result<()> {
    let mut state = State::new();
    parse_args(&mut state);
    for file_path in read_input() {
        let last_modified = get_last_modified(&file_path);
        state.files.insert(file_path, last_modified);
    }
    loop {
        thread::sleep(Duration::from_millis(100));
        if state.check() {
            let mut child = Command::new(&state.command)
                .args(&state.command_args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect(format!("Could not initiate command: {}", state.command).as_str());
            let mut stdout = child.stdout.take().unwrap();

            let mut buf = Vec::new();

            stdout
                .read_to_end(&mut buf)
                .expect("Failed to read child's stdout");
            std::io::stdout()
                .write(&buf)
                .expect("Failed to write child's stdout");
        }
    }
}
