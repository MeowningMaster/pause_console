use std::io::{stdin, stdout, Read, Write};

pub fn execute() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}