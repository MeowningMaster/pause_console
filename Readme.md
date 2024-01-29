# Description

Crate that brings functionality similar to c++'s `system("pause")`

# Usage

```rust
use pause_console::pause_console;

// print "Press Enter to continue..." then wait till Enter pressed
pause_console!();

// same with custom message
pause_console!("Sample message");

// and you can format and output message
pause_console!("Sample message: {}", 1);
```

# Caveats
`pause_console!` does not prevent the user from entering characters other than `Enter`, they will be visible. This library is good for rapid prototyping or internal projects. For better user experience use [crossterm](https://lib.rs/crates/crossterm).