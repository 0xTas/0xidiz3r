use std::io::{self, Write};

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use batch::BatchObfuscator;

pub mod batch;

#[derive(Debug)]
pub enum CharSet {
    FullSet,
    Letters,
    BadChars
}

impl CharSet {
    fn value(&self) -> Vec<char> {
        match *self {
            CharSet::FullSet => vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u',
                            'v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q',
                            'R','S','T','U','V','W','X','Y','Z','0','1','2','3','4','5','6','7','8','9','!','"','#',
                            '$','%','&','\'','(',')','*','+',',','-','.','/',':',';','<','=','>','?','@','[','\\',
                            ']','^','_','`','{','|','}','~',' ', '\n', '\r'],

            CharSet::Letters => vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u',
                            'v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q',
                            'R','S','T','U','V','W','X','Y','Z'],

            CharSet::BadChars => vec!['<','>','|','%','^','&', '\n', '\r']
        }
    }
}


/// Returns a string of a random length between min/max containing random ascii letters (mixed case).<br>
/// Min default value is 69.<br>
/// Max default value is 209 (*exceeding this value is not recommended*).<br>
/// Call with None (e.g **generate_random_chars(None, None)** to use default values).<br>
/// **Using default values is recommended unless you intend to make them smaller.**
pub fn generate_random_chars(min: Option<u32>, max: Option<u32>) -> String {
    // Functionally-default values for min and max lengths.
    let min_len: u32 = min.unwrap_or(69);
    let max_len: u32 = max.unwrap_or(209);

    let mut rng_chars: Vec<char> = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..=thread_rng().gen_range(min_len..=max_len) {
        rng_chars.push(*CharSet::Letters.value().choose(&mut rng).expect("CharSet should not be empty!"));
    };

    let rng_string: String = rng_chars.into_iter().collect();

    rng_string
}

/// Returns an obfuscated string representing a variable definition statement in Batch.
pub fn define_batch_variable(name: String, value: String, prelude: &BatchObfuscator) -> String {
    format!("%{}%%{}%{}%{}%{}", prelude.set_str, prelude.space_str, name, prelude.eq_str, value)
}


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


#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn test_charset() {
        println!("Full_Charset: {:#?}\n Letters: {:#?}\n, BadChars: {:#?}", CharSet::FullSet.value(), CharSet::Letters.value(), CharSet::BadChars.value());
    }

    #[test]
    fn test_prelude() {
        let obfuscator: BatchObfuscator = BatchObfuscator::new(None, None);

        println!("{:#?}", obfuscator);
    }

    #[test]
    fn test_collisions() {
        for _ in 0..10 {
            let mut used_strings: Vec<String> = Vec::new();
            let mut collisions: Vec<String> = Vec::new();
            #[allow(non_snake_case)]
            for N in 0..=10000 {
                let random_chars: String = generate_random_chars(None, None);

                if used_strings.contains(&random_chars) {
                    collisions.push(random_chars);
                }else {
                    used_strings.push(random_chars);
                }
                print!("Iteration: {}\r", N);
                std::io::stdout().flush().expect("Write to console failed!");
            };
            println!("Number of Collisions: {}", collisions.len());
        };
    }
}