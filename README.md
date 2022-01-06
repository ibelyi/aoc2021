## About
Solutions to [Advent of code 2021](https://adventofcode.com/2021) implemented in [Rust](https://doc.rust-lang.org/book/title-page.html). The general development environment was borrowed from https://github.com/LeoTheMighty/aoc2021 and slightly adjusted. The solutions were rarely implemented on the day of the challenge and often updated afterwards while learning more about the language.
## Running
To run solution for each day line 3 and 5 in [main.rs](src/main.rs#L3-L5) needs to be modified accordingly. One for module inclusion and one for `DAY` reference:
```rust
use aoc2021::{
    common::{lines_from_file, Step},
    day25::{solution, test_result},
};
const DAY: &str = "day25";
```
After that use `cargo run` to run and see solutions for input stored in `input.txt` for the corresponding day.
For day 23 another option is to run `cargo run --features=solution` to print solution for each case.
