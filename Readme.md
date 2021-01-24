## Description
Crate that brings functionality similar to c++'s `system("pause")`

## Usage
```rust
use pause_console;
...
//pause with "Press Enter to continue..." message
pause_console::execute();

//pause and newline
pause_console::executeln();

//pause with custom message
pause_console::execute_with_message("Custom message");

//pause with custom message and newline
pause_console::executeln_with_message("Custom message");
```
