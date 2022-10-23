use std::io::{self, Write};
pub mod batch;


/* General Utility Functions */

/// Prompts the user for input and returns it as a String.<br><br>
/// The *prompt: &str* parameter is printed without a trailing newline by default. Supply your own if desired.
pub fn input(prompt: &str) -> String {
    let mut user_input: String = String::new();

    print!("{}", prompt);
    io::stdout().flush().expect("Write to console failed!");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Read from stdin failed!");

    user_input
}
