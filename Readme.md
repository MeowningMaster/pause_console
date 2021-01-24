## Description
Crate that brings functionality similar to c++'s `system("pause")`

## Usage
```rust
use pause_console;
...
//pause with "Press Enter to continue..." message
pause_console::execute();

//pause on a new line
pause_console::executeln();

//pause with custom message
pause_console::execute_with_message("Custom message");
//or
let message = String::from("Custom message");
pause_console::execute_with_message(message.as_str());

//pause with custom message on a new line
pause_console::executeln_with_message("Custom message");
```
