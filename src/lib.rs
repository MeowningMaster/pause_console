use std::io::{stdin, stdout, Read, Write};

pub fn pause_with_message(message: String) {
    let mut stdout = stdout();
    stdout.write(b"\n").unwrap();
    stdout.write(message.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub fn pause() {
    pause_with_message(String::from("Press Enter to continue..."));
}
