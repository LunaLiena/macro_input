/// # Asynchronous Input Processing Module
///
/// This module provides an asynchronous version of the `input!` macro,
/// implemented as the `input_async!` macro, which allows collecting and validating user input
/// in asynchronous contexts (e.g., with Tokio).
///
/// ## Features
///
/// - Asynchronous input handling using `tokio::io`.
/// - Supports types implementing the `FromStr` trait.
/// - Gracefully handles invalid input by repeatedly prompting the user.
/// - Optional custom error handler.
///
/// ## Examples
///
/// Basic usage:
///
/// ```no_run
/// use macro_input::input_async;
///
/// #[tokio::main]
/// async fn main() {
///     let mut number: i32 = 0;
///     input_async!(number, "Enter a number", i32);
///     println!("You entered: {}", number);
/// }
/// ```
///
/// With a custom error handler:
///
/// ```no_run
/// use macro_input::input_async;
///
/// #[tokio::main]
/// async fn main() {
///     let mut value: f64 = 0.0;
///
///     input_async!(value, "Enter a real number", f64, |err| {
///         eprintln!("Input error: {}. Please try again.", err);
///     });
///
///     println!("You entered: {}", value);
/// }
/// ```
///
/// ## Notes
///
/// - Requires the Tokio runtime (version 1.45 or later).
/// - Only supports `stdin` input via `tokio::io::stdin()`.
/// - The input type must implement `FromStr`.

#[macro_export]
macro_rules! input_async {
    ($field:expr,$desc:expr,$ty:ty,$on_error:expr) => {{
        use std::str::FromStr;
        use tokio::io::{self, AsyncBufReadExt, BufReader};

        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut buffer = String::new();

        loop {
            print!("{} ({}): ", $desc, stringify!($ty));
            use std::io::Write;
            std::io::stdout().flush().unwrap();

            buffer.clear();
            if let Err(err) = reader.read_line(&mut buffer).await {
                eprintln!("Input read error: {}", err);
                $on_error(err);
                continue;
            }

            let input = buffer.trim();
            match <$ty>::from_str(input) {
                Ok(val) => {
                    $field = val;
                    break;
                }
                Err(e) => {
                    eprintln!(
                        "Invalid input '{}'. Expected type: {}. Error: {}",
                        input,
                        stringify!($ty),
                        e
                    );
                    $on_error(e);
                }
            }
        }
    }};

    ($field:expr, $desc:expr, $ty:ty) => {{
        use std::str::FromStr;
        use tokio::io::{self, AsyncBufReadExt, BufReader};

        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut buffer = String::new();

        loop {
            print!("{} ({}): ", $desc, stringify!($ty));
            use std::io::Write;
            std::io::stdout().flush().unwrap();

            buffer.clear();
            if let Err(err) = reader.read_line(&mut buffer).await {
                eprintln!("Input read error: {}", err);
                continue;
            }

            let input = buffer.trim();
            match <$ty>::from_str(input) {
                Ok(val) => {
                    $field = val;
                    break;
                }
                Err(e) => {
                    eprintln!(
                        "Invalid input '{}'. Expected type: {}. Error: {}",
                        input,
                        stringify!($ty),
                        e
                    );
                }
            }
        }
    }};
}
