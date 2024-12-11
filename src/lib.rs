/// Module for processing user input.
///
/// Example:
///
/// ```rust
/// use input::input;
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
///
///     // Define a variable for floating-point input
///     let mut float_value: f64 = 0.0;
///
///     // Call the macro with a custom error handler for invalid input
///     input!(float_value, "Enter a float value", f64, |err| {
///         eprintln!("Custom handler: Failed to parse input -> {}", err);
///     });
///
///     // Output the entered floating-point value
///     println!("You entered: {}", float_value);
/// }
/// ```
///
/// Macro `input!` for user input handling.
///
/// Example usage without custom error handler:
/// ```rust
/// fn main() {
///     // Define a variable to store the input value
///     let mut number: i32;
///
///     // Call the macro to prompt the user for input (no custom error handler)
///     input!(number, "Enter a number", i32);
///
///     // Output the entered value
///     println!("You entered: {}", number);
/// }
/// ```
///
/// Example usage with custom error handler:
/// ```rust
/// fn main() {
///     // Define a variable for floating-point input
///     let mut height: f64 = 0.0;
///
///     // Call the macro with a custom error handler for invalid input
///     input!(height, "Enter your height", f64, |err| {
///         eprintln!("Custom handler: Failed to parse input -> {}", err);
///     });
///
///     // Output the entered floating-point value
///     println!("You entered: {}", height);
/// }
/// ```

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
                std::io::stdout().flush().unwrap(); // Вывод приглашения для ввода
                
                let mut buffer = String::new();
                if let Err(err) = io::stdin().read_line(&mut buffer) {
                    eprintln!("input read error: {}", err);
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
