use std::{collections::{HashMap, HashSet}, fs::File, io::Write};
use crate::generate_random_chars;


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


// TODO: Handle batch command length limit of 8191 bytes with dynamic payload-length adjustments?
#[derive(Debug)]
pub struct BatchObfuscator {
    pub set_str: String,
    pub space_str: String,
    pub eq_str: String,
    pub used_variable_strings: HashSet<String>,
    pub alphabet: HashMap<char, String>,
    pub prep_commands: Vec<String>,
    pub exec_commands: Vec<String>,
    pub obfuscated_code: String,
    initialized: bool,
}

impl BatchObfuscator {
    pub fn new() -> Self {
        BatchObfuscator { 
            set_str: String::new(),
            space_str: String::new(),
            eq_str: String::new(),
            used_variable_strings: HashSet::new(),
            alphabet: HashMap::new(),
            prep_commands: Vec::new(),
            exec_commands: Vec::new(),
            obfuscated_code: String::new(),
            initialized: false,
        }
    }

    pub fn initialize(&mut self, min: Option<u32>, max: Option<u32>, src: &str) {

        self.set_str = generate_random_chars(min, max, &self.used_variable_strings);
        self.space_str = generate_random_chars(min, max, &self.used_variable_strings);
        self.eq_str = generate_random_chars(min, max, &self.used_variable_strings);
        self.build_alphabet();

        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIHdhcyBvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I="));
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIGNhbiBiZSBwcm9ncmFtYXRpY2FsbHkgZGVvYmZ1c2NhdGVkIChzb29u4oSiKSB2aWEgaHR0cHM6Ly9naXRodWIuY29tLzB4VGFzLzB4aWRpejNy"));
        self.prep_commands.push(String::from("@echo off"));
        self.prep_commands.push(format!("set {}=set", self.set_str));
        self.prep_commands.push(format!("%{}% {}= ", self.set_str, self.space_str));
        self.prep_commands.push(format!("%{}%%{}%{}==", self.set_str, self.space_str, self.eq_str));

        for chr in src.chars() {

            if !CharSet::FullSet.values().contains(&chr) {
                self.exec_commands.push(format!("{}", chr.to_owned()));
            }else if !CharSet::BadChars.values().contains(&chr) {
                let varname: &String = self.alphabet.get(&chr).expect("Key not in alphabet!");

                self.prep_commands.push(BatchObfuscator::define_batch_variable(format!("{}", varname.to_owned()), format!("{}", chr.to_owned()), &self));
                self.exec_commands.push(format!("%{}%", varname.to_owned()));
            }else {
                self.exec_commands.push(format!("{}", chr.to_owned()));
            };
        };

        let exec_string: String = self.exec_commands.join("");
        self.prep_commands.push(exec_string);
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIHdhcyBvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I="));
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIGNhbiBiZSBwcm9ncmFtYXRpY2FsbHkgZGVvYmZ1c2NhdGVkIChzb29u4oSiKSB2aWEgaHR0cHM6Ly9naXRodWIuY29tLzB4VGFzLzB4aWRpejNy"));

        self.obfuscated_code = self.prep_commands.join("\n");
        self.initialized = true;
    }

    pub fn write_obfuscated_script(&self, file_name: Option<String>) -> String {

        if !self.initialized { panic!("Obfuscator must first be initialized!"); };

        let handle_name: String = file_name.unwrap_or(String::from("obfuscated.bat"));
        let handle_clone: String = handle_name.clone();

        let mut file = File::create(handle_clone.as_str()).expect("Failed to create file!");
        file.write_all(self.obfuscated_code.as_bytes()).expect("Failed writing to file!");

        handle_name
    }

    pub fn print_obfuscated_code(&self) {
        if self.initialized {
            println!("{}", self.obfuscated_code);
        }else {
            println!("Obfuscator has not been initialized!");
        };
    }

    /* Utility */

    fn build_alphabet(&mut self) {

        for chr in CharSet::FullSet.values() {

            if !CharSet::FullSet.values().contains(&chr) {
                self.alphabet.insert(chr, format!("{}", chr));
            }else if !CharSet::BadChars.values().contains(&chr) {
                let varname: String = generate_random_chars(None, None, &self.used_variable_strings);
                self.alphabet.insert(chr, varname);
            }else {
                self.alphabet.insert(chr, format!("{}", chr));
            };
        };
    }

    fn define_batch_variable(name: String, value: String, prelude: &BatchObfuscator) -> String {
        format!("%{}%%{}%{}%{}%{}", prelude.set_str, prelude.space_str, name, prelude.eq_str, value)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charset() {
        println!("Full_Charset: {:#?}\n Letters: {:#?}\n, BadChars: {:#?}", CharSet::FullSet.values(), CharSet::Letters.values(), CharSet::BadChars.values());
    }

    #[test]
    fn test_prelude() {
        let obfuscator: BatchObfuscator = BatchObfuscator::new();

        println!("{:#?}", obfuscator);
    }
}