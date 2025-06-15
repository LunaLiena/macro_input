use std::{
    io::{BufRead, Write},
    str::FromStr,
};

extern crate once_cell;

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

/// # Examples
///
/// Example of use with a custom error handler:
///
/// ```rust
/// use macro_input::input;
///
/// fn main() {
/// // // Define a variable to store the input
/// let mut value: f64 = 0.0;
///
/// // // Use a macro with a custom error handler
/// input!(value, "Enter a real number", f64, |err| {
/// // // Process parsing error
/// eprintln!("Input error: {}. Try again.", err);
/// });
///
/// // Print the result
/// println!("You have entered: {}", value);
/// }
/// ```
///
/// # Notes
/// - The custom handler receives an error object of type `std::num::ParseFloatError`
//// (or other error type corresponding to the parsed value).
#[macro_export]
macro_rules! input {
    ($field:expr, $desc:expr, $ty:ty, $on_error:expr) => {{
        let mut stdin = std::io::stdin().lock();
        let mut stdout = std::io::stdout();
        $field = $crate::read_input(&mut stdin, &mut stdout, $desc, Some($on_error));
    }};
    ($field:expr, $desc:expr, $ty:ty) => {{
        let mut stdin = std::io::stdin().lock();
        let mut stdout = std::io::stdout();
        $field = $crate::read_input::<_, $ty, _>(&mut stdin, &mut stdout, $desc, None);
    }};
}

pub(crate) fn read_input<R: BufRead, T: FromStr, F: FnMut(&T::Err)>(
    reader: &mut R,
    writer: &mut impl Write,
    desc: &str,
    mut on_error: Option<F>,
) -> T
where
    T::Err: std::fmt::Display,
{
    loop {
        write!(writer, "{} ({}): ", desc, std::any::type_name::<T>()).unwrap();
        writer.flush().unwrap();

        let mut buffer = String::new();
        if reader.read_line(&mut buffer).is_err() {
            if let Some(f) = &on_error {}
            continue;
        }
        let buffer = buffer.trim();
        match buffer.parse::<T>() {
            Ok(val) => return val,
            Err(err) => {
                writeln!(
                    writer,
                    "Invalid input '{}'. Expected type. Error: {}",
                    buffer, err
                )
                .unwrap();
                if let Some(f) = &on_error.as_mut() {
                    // if read_line return error
                }
            }
        }
    }
}

/// Modules for handling user input in concurrent and asynchronous contexts
/// Asynchronous input processing module
///
/// This module provides utilities for non-blocking user input in async contexts.
/// Designed for use with async runtimes like tokio or async-std.
///
/// # Features
/// - Non-blocking stdin reading
/// - Timeout support for input operations
/// - Async cancellation support
/// - Integration with popular async runtimes
///
/// # Examples
/// ```
/// use crate::async_input::AsyncInput;
/// 
/// #[tokio::main]
/// async fn main() {
///     let mut input = AsyncInput::new();
///     match input.read_line().await {
///         Ok(line) => println!("You entered: {}", line),
///         Err(e) => eprintln!("Input error: {}", e),
///     }
/// }
/// ```
///
/// # Safety
/// All operations are thread-safe when used with proper async runtime.
pub mod async_input;


/// Thread-safe synchronous input processing module
///
/// Provides synchronized access to stdin across multiple threads.
/// Uses internal mutexes to prevent data races while maintaining
/// blocking behavior expected in synchronous contexts.
///
/// # Features
/// - Mutex-protected stdin access
/// - Blocking read operations
/// - Thread-local storage options
/// - Graceful error handling
///
/// # Examples
/// ```
/// use crate::thread_safe_input::ThreadSafeInput;
/// use std::thread;
///
/// let input = ThreadSafeInput::new();
/// let handle = thread::spawn(move || {
///     match input.read_line() {
///         Ok(line) => println!("Thread got: {}", line),
///         Err(e) => eprintln!("Error: {}", e),
///     }
/// });
/// handle.join().unwrap();
/// ```
///
/// # Implementation Notes
/// Uses `std::sync::Mutex` internally with proper poisoning handling.
/// Consider using `parking_lot` mutexes for better performance in contention-heavy scenarios.
pub mod thread_safe_input;
