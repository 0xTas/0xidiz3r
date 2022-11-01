/* Copyright (c) 2022 Zach Griffin (0xTas)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */


use rand::{
    Rng,
    thread_rng,
    prelude::SliceRandom
};
use std::collections::HashSet;

pub mod obfuscator;
pub mod deobfuscator;


#[derive(Debug)]
pub enum CharSet {
    /// A character set equivalent to Python's `string.ascii_letters + string.digits + string.punctuation`.
    FullSet,
    /// A character set equivalent to Python's `string.ascii_letters`.
    Letters,
    /// A character set containing bad characters that break the terminal when obfuscated on their own.
    BadChars,
}

impl CharSet {

    /// Returns a `Vec<char>` containing a character set for use in batch (de)obfuscation.
    pub fn values(&self) -> Vec<char> {
        match *self {
            CharSet::FullSet => vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u',
                            'v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q',
                            'R','S','T','U','V','W','X','Y','Z','0','1','2','3','4','5','6','7','8','9','!','"','#',
                            '$','%','&','\'','(',')','*','+',',','-','.','/',':',';','<','=','>','?','@','[','\\',
                            ']','^','_','`','{','|','}','~',' ', '\n', '\r'],

            CharSet::Letters => vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u',
                            'v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q',
                            'R','S','T','U','V','W','X','Y','Z'],

            CharSet::BadChars => vec!['<','>','|','%','^','&', '\n', '\r'],
        }
    }
}


/* Batch Utility Functions */

/// Returns a string of a random length between min/max containing random ascii letters (mixed case).<br><br>
/// Call with *min* or *max* set to *None* to use default values.<br>
/// Min default value is (7), Max default value is (109).<br><br>
/// Batch has a single-line limit of **8191**, so keep this in mind when changing these values.<br><br>
/// Shorter commands can use larger values to generate more noise.<br>
/// Longer commands run the risk of breaking in the terminal if the obfuscated length exceeds the limit.
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