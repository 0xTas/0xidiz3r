use std::{collections::{HashMap, HashSet}, fs::File, io::Write};
use crate::batch::{generate_random_chars, CharSet};


// TODO: Handle batch command length limit of 8191 bytes with dynamic payload-length adjustments?

/// **An object that generates obfuscated batch commands from un-obfuscated source commands.**
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

    /// Creates a new, empty instance of a BatchObfuscator.
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

    /// Initializes an empty BatchObfuscator, builds an obfuscated alphabet, and uses it to obfuscate the provided source code.<br><br>
    /// *Min/Max* refer to length constraints on the obfuscated variable names.<br>
    /// Call with *min* or *max* set to *None* to use default values.<br>
    /// Min default value is (**7**), Max default value is (**109**).<br><br>
    /// Batch has a single-line limit of **8191**, so keep this in mind when changing these values.<br><br>
    /// Shorter commands can use larger values to generate more noise.<br>
    /// Longer commands run the risk of breaking in the terminal if the obfuscated length exceeds the limit.
    pub fn initialize(&mut self, min: Option<u32>, max: Option<u32>, src: String) {

        // Create obfuscated variables for the set keyword, the space character, and the assignment operator.
        self.set_str = generate_random_chars(min, max, &self.used_variable_strings);
        self.space_str = generate_random_chars(min, max, &self.used_variable_strings);
        self.eq_str = generate_random_chars(min, max, &self.used_variable_strings);

        // Insert base64-encoded watermarks to assist any potential deobfuscation attempts in the wild.
        // Write the script header defining an obfuscated way of assigning further variables.
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIHdhcyBvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I="));
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIGNhbiBiZSBwcm9ncmFtYXRpY2FsbHkgZGVvYmZ1c2NhdGVkIChzb29u4oSiKSB2aWEgaHR0cHM6Ly9naXRodWIuY29tLzB4VGFzLzB4aWRpejNy"));
        self.prep_commands.push(String::from("@echo off"));
        self.prep_commands.push(format!("set {}=set", self.set_str));
        self.prep_commands.push(format!("%{}% {}= ", self.set_str, self.space_str));
        self.prep_commands.push(format!("%{}%%{}%{}==", self.set_str, self.space_str, self.eq_str));

        // Build an obfuscated alphabet with variables and push their assignment statements into the prep_commands Vec.
        self.build_alphabet();


        self.obfuscate(src);

        self.initialized = true;
    }

    /// Writes the obfuscated source of a pre-initialized BatchObfuscator to a file, and returns a string containing the name of that file.<br><br>
    /// Output filename defaults to *obfuscated.bat* when **None** is passed into the parameter.<br><br>
    /// **This method panics if file creation/writing fails.**
    pub fn write_obfuscated_script(&self, file_name: Option<String>) -> String {

        if !self.initialized { panic!("Obfuscator must first be initialized!"); };

        let handle_name: String = file_name.unwrap_or(String::from("obfuscated.bat"));
        let handle_clone: String = handle_name.clone();

        let mut file = File::create(handle_clone.as_str()).expect("Failed to create file!");
        file.write_all(self.obfuscated_code.as_bytes()).expect("Failed writing to file!");

        handle_name
    }


    /* Utility */

    /// Builds an obfuscated alphabet using the Batch obfuscation character set.
    fn build_alphabet(&mut self) {

        for chr in CharSet::FullSet.values() {

            if !CharSet::BadChars.values().contains(&chr) {
                let varname: String = generate_random_chars(None, None, &self.used_variable_strings);
                self.alphabet.insert(chr, varname.clone());

                if !self.prep_commands.contains(&BatchObfuscator::define_batch_variable(format!("{}", varname.to_owned()), format!("{}", chr.to_owned()), &self)) {
     
                    self.prep_commands.push(BatchObfuscator::define_batch_variable(format!("{}", varname.to_owned()), format!("{}", chr.to_owned()), &self));
                };
            }else {
                self.alphabet.insert(chr, format!("{}", chr));
            };
        };
    }

    fn obfuscate(&mut self, src: String) {

        // Reassemble input source using obfuscated alphabet variables.
        for chr in src.chars() {

            if !CharSet::FullSet.values().contains(&chr) {
                self.exec_commands.push(format!("{}", chr.to_owned()));
            }else if !CharSet::BadChars.values().contains(&chr) {
                let varname: &String = self.alphabet.get(&chr).expect("Key not in alphabet!");
                self.exec_commands.push(format!("%{}%", varname.to_owned()));
            }else {
                self.exec_commands.push(format!("{}", chr.to_owned()));
            };
        };

        // Convert obfuscated output to a string and append it to the prep_commands Vec.
        let exec_string: String = self.exec_commands.join("");
        self.prep_commands.push(exec_string);
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIHdhcyBvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I="));
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIGNhbiBiZSBwcm9ncmFtYXRpY2FsbHkgZGVvYmZ1c2NhdGVkIChzb29u4oSiKSB2aWEgaHR0cHM6Ly9naXRodWIuY29tLzB4VGFzLzB4aWRpejNy"));

        // Join the obfuscated output on newlines and complete the initialization.
        self.obfuscated_code = self.prep_commands.join("\n");
    }

    /// Returns a string representing an obfuscated variable definition statement in Batch.
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