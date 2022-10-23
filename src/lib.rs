use std::io::{self, Write};
use std::collections::HashSet;
use rand::{thread_rng, Rng, prelude::SliceRandom};
use batch::CharSet;

pub mod batch;


/* General Utility Functions */

/// Returns a string of a random length between min/max containing random ascii letters (mixed case).<br>
/// Min default value is 69.<br>
/// Max default value is 209 (*exceeding this value is not recommended*).<br>
/// Call with None (e.g **generate_random_chars(None, None)** to use default values).<br>
/// **Using default values is recommended unless you intend to make them smaller.**
pub fn generate_random_chars(min: Option<u32>, max: Option<u32>, used: &HashSet<String>) -> String {
    // Functionally-default values for min and max lengths.
    let min_len: u32 = min.unwrap_or(7);
    let max_len: u32 = max.unwrap_or(109);

    let mut rng_chars: Vec<char> = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..=thread_rng().gen_range(min_len..=max_len) {
        rng_chars.push(*CharSet::Letters.values().choose(&mut rng).expect("CharSet should not be empty!"));
    };

    let rng_string: String = rng_chars.into_iter().collect();

    if !used.contains(&rng_string) {
        return rng_string;
    }else {
        return generate_random_chars(Some(min_len), Some(max_len), used);
    };
}

pub fn input(prompt: &str) -> String {
    let mut user_input: String = String::new();

    print!("{}", prompt);
    io::stdout().flush().expect("Write to console failed!");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Read from stdin failed!");

    user_input
}
