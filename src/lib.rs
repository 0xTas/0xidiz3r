use std::io::{self, Write};
pub mod batch;


/* General Utility Functions */

pub fn input(prompt: &str) -> String {
    let mut user_input: String = String::new();

    print!("{}", prompt);
    io::stdout().flush().expect("Write to console failed!");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Read from stdin failed!");

    user_input
}
