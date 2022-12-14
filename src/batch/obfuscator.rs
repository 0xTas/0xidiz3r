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


use regex::Regex;
use std::{
    fs::File,
    io::Write,
    process::exit,
    collections::{
        HashMap,
        HashSet
    }
};
use crate::{
    input,
    batch::{
        CharSet,
        generate_random_chars
    }
};


/// ### An object that generates obfuscated batch commands from un-obfuscated source commands.<br>
/// ### Example Usage:<br>
/// ```
/// use oxidizer::batch::obfuscator::BatchObfuscator;
/// 
/// let poc = "start C:/Windows/System32/calc.exe";
/// let mut obfuscator = BatchObfuscator::new();
/// obfuscator.initialize(None, None, poc.to_string());
/// 
/// let obfuscated_script = obfuscator.write_obfuscated_script(None);
/// 
/// // prints: "Obfuscated code was written to: obfuscated.bat".
/// println!("Obfuscated code was written to: {}", obfuscated_script);
#[derive(Debug, Default)]
pub struct BatchObfuscator {
    pub set_str: String,
    pub space_str: String,
    pub eq_str: String,
    pub used_variable_strings: HashSet<String>,
    pub alphabet: HashMap<char, String>,
    pub prep_commands: Vec<String>,
    pub exec_commands: Vec<String>,
    pub obfuscated_code: String,
    echo_mode: bool,
    warn_mode: bool,
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
            echo_mode: false,
            warn_mode: true,
            initialized: false,
        }
    }

    /// Prevents from writing boiler-plate "@echo off" to output script.
    pub fn dont_echo(&mut self) {
        self.echo_mode = true;
    }

    /// Disables warnings about user-defined variables in input scripts.
    pub fn dont_warn(&mut self) {
        self.warn_mode = false;
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
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIGNhbiBiZSBwcm9ncmFtYXRpY2FsbHkgZGVvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I="));
        if self.echo_mode {self.prep_commands.push(String::from("@echo off"));};
        self.prep_commands.push(format!("set {}=set", self.set_str));
        self.prep_commands.push(format!("%{}% {}= ", self.set_str, self.space_str));
        self.prep_commands.push(format!("%{}%%{}%{}==", self.set_str, self.space_str, self.eq_str));

        // Build an obfuscated alphabet with variables and push their assignment statements into the prep_commands Vec.
        self.build_alphabet(min, max);

        // Obfuscate the cleartext source code using our newly-created obfuscated alphabet.
        self.obfuscate(src);

        self.initialized = true;
    }

    /// Writes the obfuscated source of a pre-initialized BatchObfuscator to a file, and returns a string containing the name of that file.<br><br>
    /// Output filename defaults to *obfuscated.bat* when **None** is passed into the parameter.<br><br>
    /// **This method panics if file creation/writing fails.**
    pub fn write_obfuscated_script(&self, file_name: Option<String>) -> String {

        if !self.initialized {
            eprintln!("\nObfuscator must first be initialized!");
            exit(1);
        };

        let handle_name: String = file_name.unwrap_or_else(|| String::from("obfuscated.bat"));
        let handle_clone: String = handle_name.clone();

        let mut file = File::create(handle_clone.as_str()).expect("Failed to create file!");
        file.write_all(self.obfuscated_code.as_bytes()).expect("Failed writing to file!");

        handle_name
    }


    /* Utility */

    /// Builds an obfuscated alphabet using the Batch obfuscation character set.
    fn build_alphabet(&mut self, min: Option<u32>, max: Option<u32>) {

        for chr in CharSet::FullSet.values() {
        
            if !CharSet::BadChars.values().contains(&chr) {
                let varname: String = generate_random_chars(min, max, &self.used_variable_strings);
                self.alphabet.insert(chr, varname.clone());

                if !self.prep_commands.contains(&BatchObfuscator::define_batch_variable(
                                            varname.to_owned().to_string(),
                                            format!("{}", chr.to_owned()),
                                            self)) 
                {
                    self.prep_commands.push(BatchObfuscator::define_batch_variable(
                                            varname.to_owned().to_string(),
                                            format!("{}", chr.to_owned()),
                                            self));
                };
            }else {
                self.alphabet.insert(chr, format!("{}", chr));
            };
        };
    }

    /// Obfuscates cleartext batch commands using an obfuscated alphabet of variables.
    fn obfuscate(&mut self, src: String) {

        let match_variable_lines: Regex = Regex::new("%[a-zA-Z0-9_-~!@#$^&/.,<>;:'\"=]+%").expect("Regex not valid!");
        let match_set_lines: Regex = Regex::new("set .+=.+").expect("Regex not valid!");
        let src_list: Vec<&str> = src.split('\n').collect();
        let mut warned: bool = false;

        for line in src_list {

            // Find char indicies for any occurances of '%' in a line.
            let find_percent_index = || {
                let mut perc_index: Vec<usize> = Vec::new();
                for (i, c) in line.char_indices() {
                    if c == '%' {
                        perc_index.push(i);
                    };
                };

                if perc_index.is_empty() { return None };

                Some(perc_index)
            };

            if line.contains('%') && !match_variable_lines.is_match(line) {

                let perc_index: Vec<usize> = find_percent_index().expect("No percent symbols in sample!");

                let mut skip: bool = false;
                for (i, c) in line.char_indices() {

                    if skip {
                        skip = false;
                        continue;
                    };

                    if perc_index.contains(&i) {
                        let blob: &str = &line[i..=i+1];
                        let mut obfuscate_blob = || {
                            let varname: String = generate_random_chars(None, None, &self.used_variable_strings);
                            let varline: String = BatchObfuscator::define_batch_variable(varname.clone(), blob.to_string(), self);

                            self.prep_commands.push(varline);
                            self.exec_commands.push(format!("%{}%", varname));
                        };
                        obfuscate_blob();
                        skip = true;
                        continue;
                    };

                    if !CharSet::FullSet.values().contains(&c) {
                        self.exec_commands.push(format!("{}", c.to_owned()));
                    }else if !CharSet::BadChars.values().contains(&c) {
                        let varname: &String = self.alphabet.get(&c).expect("Key not in alphabet!");
                        self.exec_commands.push(format!("%{}%", varname.to_owned()));
                    }else {
                        self.exec_commands.push(format!("{}", c.to_owned()));
                    };
                };

            // If the input script contains custom/environment vars, warn about this method's limitations.
            }else if match_variable_lines.is_match(line) ||
                (line.starts_with(':') && !line.starts_with("::")) || 
                (match_set_lines.is_match(line) && line.to_lowercase().starts_with("set")) {

                let mut heed: String = String::new();
                if !warned && self.warn_mode {
                    println!("\n[!]--> WARNING: Because of the way this obfuscation method works, 
                        variables you define or use in your scripts, including environment variables,
                        and function labels, cannot be effectively obfuscated using this obfuscation method, 
                        and lines containing them will be printed as-is in order to preserve functionality.");

                    heed = input("\nContinue Anyway? [Y/N] ~> ");
                };

                if !self.warn_mode || heed.to_lowercase().contains('y') || warned {
                    self.exec_commands.push(line.to_owned());
                    warned = true;
                }else {
                    println!("\nObfuscation aborted!");
                    exit(0);
                };
            }else {

                // Reassemble input source using obfuscated alphabet variables.
                for chr in line.chars() {

                    if !CharSet::FullSet.values().contains(&chr) {
                        self.exec_commands.push(format!("{}", chr.to_owned()));
                    }else if !CharSet::BadChars.values().contains(&chr) {
                        let varname: &String = self.alphabet.get(&chr).expect("Key not in alphabet!");
                        self.exec_commands.push(format!("%{}%", varname.to_owned()));
                    }else {
                        self.exec_commands.push(format!("{}", chr.to_owned()));
                    };
                };
            };

            self.exec_commands.push("\n".to_string());
        };

        // Convert obfuscated output to a string and append it to the prep_commands Vec.
        let exec_string: String = self.exec_commands.join("");
        self.prep_commands.push(exec_string);
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIHdhcyBvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I="));
        self.prep_commands.push(String::from(":: VGhpcyBmaWxlIGNhbiBiZSBwcm9ncmFtYXRpY2FsbHkgZGVvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I="));

        // Join the obfuscated output on newlines and complete the initialization.
        self.obfuscated_code = self.prep_commands.join("\n");
    }

    /// Returns a string representing an obfuscated variable definition statement in Batch.
    fn define_batch_variable(name: String, value: String, prelude: &BatchObfuscator)
    -> String {
        format!("%{}%%{}%{}%{}%{}", prelude.set_str,
                prelude.space_str, name, prelude.eq_str, value)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charset() {
        println!("Full_Charset: {:#?}\n Letters: {:#?}\n, BadChars: {:#?}",
                CharSet::FullSet.values(), CharSet::Letters.values(),
                CharSet::BadChars.values());
    }

    #[test]
    fn test_prelude() {
        let obfuscator: BatchObfuscator = BatchObfuscator::new();

        println!("{:#?}", obfuscator);
    }
}