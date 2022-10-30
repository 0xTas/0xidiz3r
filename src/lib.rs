use std::{
    time::Duration,
    io::{self, Write}, 
    thread::sleep,
};
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

/// Sleeps for the given time in milliseconds.
pub fn wait(dur: u64) {
    let dur: Duration = Duration::from_millis(dur);
    sleep(dur);
}