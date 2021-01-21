use std::io::{stdin, stdout, Write};

const DEFAULT_MESSAGE: &str = "Press Enter to continue...";

pub fn execute() {
    execute_with_message(DEFAULT_MESSAGE);
}

pub fn executeln() {
    executeln_with_message(DEFAULT_MESSAGE);
}

pub fn execute_with_message(message: &str) {
    let mut stdout = stdout();
    stdout.write(message.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read_line(&mut String::new()).unwrap();
}

pub fn executeln_with_message(message: &str) {
    let mut stdout = stdout();
    stdout.write(b"\n").unwrap();
    stdout.write(message.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read_line(&mut String::new()).unwrap();
}