use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{
    collections::HashMap,
    io::{self, Read},
    time::SystemTime,
};
use std::{env, fs, process, thread};

/// Poll every 250ms
const DEFAULT_POLL_RATE: u64 = 250;

/// The application state
#[derive(Debug)]
struct State {
    /// Map of files to watch over
    files: HashMap<String, u128>,
    /// The base command to run (e.g. "cargo")
    command: String,
    /// The base command's arguments (e.g. ["build", "--release"])
    command_args: Vec<String>,
    /// Polling rate at which to check for changes
    poll_rate: u64,
}

impl State {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
            command: String::new(),
            command_args: Vec::new(),
            poll_rate: DEFAULT_POLL_RATE,
        }
    }
    /// `Self.check` will iterate over all the entries in the `Self.files` map.
    /// When there's a difference between the recorded modified time and the actual modified time,
    /// we will update the value in the map, and return true at the end of the loop
    fn check(&mut self) -> bool {
        let mut any_changed = false;
        // iterate over the entries
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

/// `get_last_modified` takes in a file path as a string,
/// and returns the date it was last modified at as a unix time stamp
/// (i.e. time in ms from the unix epoch)
fn get_last_modified(file_path: &String) -> u128 {
    fs::metadata(file_path)
        .inspect_err(|e| eprintln!("Failed to read file: {e}"))
        .unwrap()
        .modified()
        .inspect_err(|e| eprintln!("Could not read modified time on file: {e}"))
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get time since unix epoch")
        .as_millis()
}

/// `read_standard_input` will read all the files coming in from stdin.
/// Incoming files should be seperated by a newline ("\n")
fn read_standard_input() -> Vec<String> {
    let mut input_buffer: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut input_buffer)
        .expect("Could not read stdin. See `watch --help`");
    let input_as_str = String::from_utf8(input_buffer)
        .expect("Could not parse std input as a string. See `watch --help`");
    input_as_str
        .split("\n")
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .collect()
}

/// `parse_args` will parse the arguments passed into this program.
fn parse_args(state: &mut State) {
    let args: Vec<String> = env::args().collect();
    let help_text = include_str!("../help.txt");
    match args.len() {
        1 => {
            println!("No command found\n");
            println!("{}", help_text);
            process::exit(0);
        }
        2 => {
            let arg = args[1].clone();
            if arg == "-h" || arg == "--help" {
                println!("{}", help_text);
                process::exit(0);
            }
            let split_command_string: Vec<String> = args[1]
                .clone()
                .split(" ")
                .map(str::to_string)
                .filter(|x| !x.is_empty())
                .collect();
            let command = split_command_string
                .first()
                .expect("Could not parse command. See `watch --help`");
            let command_args = split_command_string.split_at(1).1;
            state.command = command.to_owned();
            state.command_args = Vec::from(command_args);
        }
        4 => {
            let arg = args[1].clone();
            if arg != "-p" && arg != "--poll-rate" {
                println!("Invalid usage...\n");
                println!("{}", help_text);
                process::exit(1);
            }
            state.poll_rate = args[2]
                .parse()
                .expect("Could not parse poll rate value. Use an unsigned integer");

            let split_command_string: Vec<String> = args[3]
                .clone()
                .split(" ")
                .map(str::to_string)
                .filter(|x| !x.is_empty())
                .collect();
            let command = split_command_string
                .first()
                .expect("Could not parse command. See `watch --help`");
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
    for file_path in read_standard_input() {
        let last_modified = get_last_modified(&file_path);
        state.files.insert(file_path, last_modified);
    }
    
    let pluralizer = if state.files.len() == 1 {
        ""
    } else {
        "s"
    };
    println!("Watching {} file{}", state.files.len(), pluralizer);

    loop {
        thread::sleep(Duration::from_millis(state.poll_rate));
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
