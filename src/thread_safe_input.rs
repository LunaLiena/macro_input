use std::sync::{Arc, Mutex};

// Global mutext for synchronize input/output
pub static INPUT_LOCK: once_cell::sync::Lazy<Arc<Mutex<()>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(())));

/// Thread-safe macro for reading user input from the console.
///
/// `safe_input!` ensures safe input handling in a multithreaded context by using a global `Mutex`
/// (`INPUT_LOCK`) to synchronize access to standard input/output. This is useful when multiple threads
/// may prompt the user simultaneously.
///
/// # Features
/// - Thread-safe I/O via a global `Mutex`
/// - Graceful error handling with input retry
/// - Supports custom error handlers
/// - Accepts any type that implements `FromStr`
///
/// # Syntax
/// ```rust
/// safe_input!(variable, "Prompt text", Type);
/// safe_input!(variable, "Prompt text", Type, |err| { custom_error_handling });
/// ```
///
/// # Examples
///
/// ## Basic usage
/// ```rust
/// use macro_input::safe_input;
///
/// fn main() {
///     let mut age: u32 = 0;
///     safe_input!(age, "Enter your age", u32);
///     println!("Age: {}", age);
/// }
/// ```
///
/// ## With a custom error handler
/// ```rust
/// use macro_input::safe_input;
///
/// fn main() {
///     let mut value: f64 = 0.0;
///     safe_input!(value, "Enter a number", f64, |err| {
///         eprintln!("Parsing error: {}", err);
///     });
///     println!("Value: {}", value);
/// }
/// ```
///
/// # Thread Safety
/// This macro uses a global `Mutex` (`INPUT_LOCK`) to synchronize access to `stdin` and `stdout`,
/// ensuring that only one thread can prompt the user at a time and preventing mixed output.
///
/// # Notes
/// - The input type must implement the `FromStr` trait.
/// - The optional error handler (4th argument) can be any closure or function accepting:
///   - a parsing error of type `<T as FromStr>::Err`, or
///   - a `std::io::Error` if reading the line fails.
/// - The macro loops until valid input is entered.
///
/// # Related
/// - [`input!`] — a similar macro without thread safety (not synchronized)
/// - [`INPUT_LOCK`] — the global `Mutex` used to ensure thread-safe I/O
#[macro_export]
macro_rules! safe_input {
    ($field:expr,$desc:expr,$ty:ty,$on_error:expr) => {{
        use std::io::{self, Write};
        use std::str::FromStr;

        let _lock = $crate::INPUT_LOCK.lock().unwrap();
        loop {
            print!("{} ({}): ", $desc, stringify!($ty));
            io::stdout().flush().unwrap();

            let mut buffer = String::new();
            if let Err(err) = io::stdin().read_line(&mut buffer) {
                eprintln!("Failed to read line: {}", err);
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
        use std::io::{self, Write};
        use std::str::FromStr;

        // Захватываем глобальный мьютекс
        let _lock = $crate::INPUT_LOCK.lock().unwrap();

        loop {
            print!("{} ({}): ", $desc, stringify!($ty));
            io::stdout().flush().unwrap();

            let mut buffer = String::new();
            if let Err(err) = io::stdin().read_line(&mut buffer) {
                eprintln!("Failed to read line: {}", err);
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
