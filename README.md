# macro_input

`macro_input` is a library for handling user input in Rust. It provides a handy `input!` macro that makes it easy to receive and process console input.

## Features

- Input support for any type that implements the `FromStr` trait (e.g. `i32`, `f64`).
- Automatic handling of input errors and repeat the request until a valid value is obtained.
- Possibility to use custom error handlers.
- Convenient interface for input processing with type and message indication.

