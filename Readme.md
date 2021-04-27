# Description
Crate that brings functionality similar to c++'s `system("pause")`

# Usage
```rust
use pause_console::*;
...

//print "Press Enter to continue..." then wait till Enter pressed
pause_console!();

//same with custom message
pause_console!("Sample message");

//same with newlines
pause_console!("\nSample message\n");
```
