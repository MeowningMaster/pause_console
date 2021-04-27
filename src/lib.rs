use std::io::{stdin, stdout, Write};

pub const PAUSE_CONSOLE_DEFAULT_MESSAGE: &str = "Press Enter to continue...";

pub fn pause_console_execute(message: &str) {
    let mut stdout = stdout();
    stdout.write(message.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read_line(&mut String::new()).unwrap();
}

#[macro_export]
macro_rules! pause_console {
    ($message: expr) => {
        pause_console_execute($message);
    };
    () => {
        pause_console_execute(PAUSE_CONSOLE_DEFAULT_MESSAGE);
    };
}