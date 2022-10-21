use std::collections::HashMap;
use crate::{generate_random_chars, CharSet};

#[derive(Debug)]
pub struct BatchObfuscator {
    pub set_str: String,
    pub space_str: String,
    pub eq_str: String,
    pub alphabet: HashMap<char, String>
}

impl BatchObfuscator {
    pub fn new(min: Option<u32>, max: Option<u32>) -> Self {
        BatchObfuscator { 
            set_str: generate_random_chars(min, max),
            space_str: generate_random_chars(min, max),
            eq_str: generate_random_chars(min, max),
            alphabet: build_alphabet()
        }
    }
}


fn build_alphabet() -> HashMap<char, String> {
    let mut alphabet: HashMap<char, String> = HashMap::new();
    for chr in CharSet::FullSet.value() {
        if !CharSet::BadChars.value().contains(&chr) {
            let varname: String = generate_random_chars(None, None);
            alphabet.insert(chr, varname);
        }else {
            alphabet.insert(chr, format!("{}", chr));
        };
    };

    alphabet
}