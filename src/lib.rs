/// Module for processing user input.
///
///
/// #Description
///This module provides the `input!` macro, which simplifies the process of collecting and validating
/// user input. The macro prompts the user for a value, validates the input, and assigns it to a variable.
///
/// /// # Features
/// - Supports input for any type that implements the `FromStr` trait (e.g., `i32`, `f64`).
/// - Handles invalid input gracefully by prompting the user to try again.
/// - Allows custom error handling via an optional callback.

///Basic usage

/// #Examples
///
///
/// ```rust
/// use macro_input::input;
///
/// fn main() {
///     // Define a variable to store the input value
///     let mut number: i32 = 0;
///
///     // Call the macro to prompt the user for input
///     input!(number, "Enter a number", i32);
///
///     // Output the entered value
///     println!("You entered: {}", number);
/// }
/// ```
///
/// # Behavior
/// The `input!` macro does not crash when invalid input is provided. Instead:
/// - It displays an error message.
/// - It prompts the user to try again until valid input is entered.
///
/// # Notes
/// - The macro relies on the `FromStr` trait for parsing, so the types used must implement `FromStr`.
/// - Custom error handlers receive the parsing error as an argument and can be used for logging or additional logic./// ```

#[macro_export]
macro_rules! input {
    ($field:expr, $desc:expr, $ty:ty, $on_error:expr) => {
        {
            use std::io::{self, Write};
            use std::str::FromStr;

            loop {
                print!("{} ({}): ", $desc,stringify!($ty));
                std::io::stdout().flush().unwrap(); 
                
                let mut buffer = String::new();
                if let Err(err) = io::stdin().read_line(&mut buffer) {
                    eprintln!("Input read error: {}", err);
                    $on_error(err);
                    continue;
                }

                buffer = buffer.trim().to_string();

                match buffer.parse::<$ty>() {
                    Ok(value) => {
                        $field = value;
                        break;
                    }
                    Err(e) => {
                        eprintln!("Invalid entry '{}'. The type {} was expected. Error: {}",buffer,stringify!($ty),e);
                        $on_error(e);
                    }
                }
            }
        }
    };
    ($field:expr, $desc:expr, $ty:ty) => {
        {
            use std::io::{self, Write};
            use std::str::FromStr;

            loop {
                print!("{} ({}): ", $desc,stringify!($ty));
                std::io::stdout().flush().unwrap();
                let mut buffer = String::new();
                if let Err(err) = io::stdin().read_line(&mut buffer) {
                    eprintln!("Input read error: {}", err);
                    continue;
                }

                buffer = buffer.trim().to_string();

                match buffer.parse::<$ty>() {
                    Ok(value) => {
                        $field = value;
                        break;
                    }
                    Err(e) => {
                        eprintln!("Incorrect input '{}'. The type {} was expected. Error: {}", buffer, stringify!($ty), e);
                    }
                }
            }
        }
    };
}
