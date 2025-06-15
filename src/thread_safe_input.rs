//! Thread-safe input handling utilities
use std::sync::{Arc, Mutex};


/// Global mutex for synchronizing console I/O across threads
///
/// This lock ensures thread-safe access to stdin/stdout when multiple threads
/// need to interact with the user simultaneously. The mutex is:
/// - Automatically initialized on first use (via `Lazy`)
/// - Wrapped in `Arc` for shared ownership
/// - Protects against interleaved output and input races
///
/// # Example
/// ```
/// use self::INPUT_LOCK;
/// use std::thread;
///
/// let handles: Vec<_> = (0..5).map(|i| {
///     thread::spawn(move || {
///         let _guard = INPUT_LOCK.lock().unwrap();
///         println!("Thread {} got the lock", i);
///         // Safe to do I/O here
///     })
/// }).collect();
///
/// for handle in handles {
///     handle.join().unwrap();
/// }
/// ```
pub static INPUT_LOCK: once_cell::sync::Lazy<Arc<Mutex<()>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(())));


/// Thread-safe macro for reading user input in concurrent applications
///
/// This macro provides synchronized input handling with these guarantees:
/// 1. Atomic I/O operations (no interleaved prompts/output)
/// 2. Thread-safe value parsing
/// 3. Graceful error recovery
///
/// # Thread Safety Model
/// - Uses global [`INPUT_LOCK`] to serialize access to stdin/stdout
/// - Each macro invocation holds the lock for the entire operation
/// - Prevents these common threading issues:
///   - Interleaved console output
///   - Stdin contention
///   - Race conditions in prompt-response flows
///
/// # Examples
///
/// ## Basic Multi-threaded Usage
/// ```rust
/// use crate::thread_safe_input::safe_input;
/// use std::thread;
///
/// let mut threads = vec![];
/// for i in 0..3 {
///     threads.push(thread::spawn(move || {
///         let mut value = 0;
///         safe_input!(value, &format!("Thread {}: Enter number", i), i32);
///         value
///     }));
/// }
///
/// for t in threads {
///     println!("Thread returned: {}", t.join().unwrap());
/// }
/// ```
///
/// ## With Error Handling
/// ```rust
/// use crate::thread_safe_input::safe_input;
/// use std::thread;
///
/// thread::spawn(|| {
///     let mut data = String::new();
///     safe_input!(data, "Enter your name", String, |e| {
///         eprintln!("Error in thread: {:?}", e);
///     });
///     println!("Hello, {}", data);
/// }).join().unwrap();
/// ```
///
/// # Performance Considerations
/// - The global lock means only one thread can do I/O at a time
/// - For high-throughput systems, consider:
///   - Dedicated I/O thread with channel communication
///   - Buffering multiple prompts before locking
/// - Lock is held only during actual I/O operations
///
/// # Panics
/// - If the mutex is poisoned (a thread panicked while holding the lock)
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
        let _lock = self::INPUT_LOCK.lock().unwrap();

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
