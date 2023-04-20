# ploop.rs
More robust and reliable than the original Bash script, with better memory management, improved error handling, and more accurate timing information.
For more information on the original project, ploop.sh, please visit `https://github.com/apple-fritter/ploop.sh`, the [ploop.sh](https://github.com/apple-fritter/ploop.sh) GitHub page.

## [ploop.sh](https://github.com/apple-fritter/ploop.sh), oxidized.

The original ploop script was written in Bash and used the `read` command to read each line of a `TSV` file and process it one at a time. This Rust program, on the other hand, reads the entire file into memory as a `Vector` type, which allows for better memory management and faster processing.

In addition, the Rust program uses Rust's built-in `timer` functionality to record the start and end times of the program's execution, rather than using the `date` command in a Bash script.

Another important change is the use of Rust's `standard error` handling mechanisms. In the original script, errors would simply be printed to the console and the program would continue running. The Rust program, however, prints errors to standard error and exits with a non-zero exit code, which is a common convention in Unix-based systems.

## Requirements
There are two external crates that are used in the code and must be included as dependencies in the `Cargo.toml` file in, order for the program to compile:

```
chrono: A crate for working with dates and times.
clap: A crate for parsing command-line arguments.
```
