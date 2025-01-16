/// ($($arg:tt)*): This is the pattern-matching part of the macro. Here’s what it does:
/// $arg: A variable name to capture input.
/// :tt: Stands for "token tree," which matches any valid Rust syntax (like expressions, identifiers, etc.).
/// $(...)*: This allows matching zero or more repetitions of the token tree. 
/// Essentially, it captures everything passed to the macro (e.g., log_message!("Port: {}", 3000);).

#[macro_export]
macro_rules! log_message {
    ($level: expr, $($arg: tt)*) => {
        $crate::logging::log_messages_to_log_file(&format!($($arg)*), $level);
    };
}

// #[macro_export]
// macro_rules! log_if_panic {
//     ($code_block: expr, $message: expr) => {
//         $code_block().unwrap_or_else(|_| log_and_panic($message))
//     };
// }

/// the below macro takes a async code block 
/// awaits the result and then matches it with 
///     # Ok() if no error
///     # Err(err) if any thing goes wrong, and writes the error to the server.log file !

#[macro_export]
macro_rules! log_if_panic_async {
    ($code_block: expr, $message: expr) => {
        match $code_block.await {
            Ok(value) => value,
            Err(err) => {
                log_messages_to_log_file(&format!("{}: {:?}", $message, err), "ERROR");
                panic!("{}: {:?}", $message, err);
            }
        }
    };
}