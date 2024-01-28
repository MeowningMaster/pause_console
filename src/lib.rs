#[macro_export]
macro_rules! pause_console {
    () => {
        std::print!("Press Enter to continue...");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::io::stdin().read_line(&mut String::new()).unwrap();
    };
    ($($arg:tt)*) => {
        std::print!($($arg)*);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::io::stdin().read_line(&mut String::new()).unwrap();
    };
}
