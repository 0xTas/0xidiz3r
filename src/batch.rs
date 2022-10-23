use std::collections::HashSet;
use rand::{thread_rng, Rng, prelude::SliceRandom};
pub mod obfuscator;


#[derive(Debug)]
pub enum CharSet {
    FullSet,
    Letters,
    BadChars,
}

impl CharSet {
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