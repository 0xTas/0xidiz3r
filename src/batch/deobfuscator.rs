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
use super::CharSet;
use std::{
    fs::File,
    io::Write,
    collections::HashMap
};


/// ### An object that generates cleartext batch commands from obfuscated source commands.<br>
/// ### Example Usage:<br>
/// ```
/// use oxidizer::batch::deobfuscator::BatchDeobfuscator;
/// 
/// let poc = r"
/// set HQTW=set
/// %HQTW% HswBh= 
/// %HQTW%%HswBh%xtYRu==
/// %HQTW%%HswBh%ubBBS%xtYRu%c
/// %HQTW%%HswBh%XRVHZ%xtYRu%e
/// %HQTW%%HswBh%qWvHi%xtYRu%h
/// %HQTW%%HswBh%XyVFS%xtYRu%o
/// %HQTW%%HswBh%BslH%xtYRu%C
/// %HQTW%%HswBh%AsBi%xtYRu%P
/// %HQTW%%HswBh%biQK%xtYRu% 
/// %XRVHZ%%ubBBS%%qWvHi%%XyVFS%%biQK%%AsBi%%XyVFS%%BslH%
/// ";
/// 
/// let mut deobfuscator = BatchDeobfuscator::new();
/// deobfuscator.initialize(poc.to_string());
/// 
/// let deobfuscated_script = deobfuscator.write_deobfuscated_script(None);
/// 
/// // prints: "Deobfuscated code was written to: deobfuscated.bat".
/// println!("Deobfuscated code was written to: {}", deobfuscated_script);
#[derive(Debug)]
pub struct BatchDeobfuscator {
    pub set_str: String,
    pub space_str: String,
    pub eq_str: String,
    pub alphabet: HashMap<String, String>,
    pub cleaned_code: String,
    initialized: bool,
}


impl BatchDeobfuscator {

    /// Creates a new, empty instance of a BatchDeobfuscator.
    pub fn new() -> Self {
        BatchDeobfuscator {
            set_str: String::new(),
            space_str: String::new(),
            eq_str: String::new(),
            alphabet: HashMap::new(),
            cleaned_code: String::new(),
            initialized: false,
        }
    }

    /// Initializes an empty BatchDeobfuscator, reverse_engineers an obfuscated alphabet, and attempts to deobfuscate the provided source code.
    pub fn initialize(&mut self, src: String) {

        // Pattern matching to identify set, space, and equals variables.
        let re_set = Regex::new("set [a-zA-Z]+=set").expect("Regex not valid!");
        let re_space = Regex::new("%[a-zA-Z]+% [a-zA-Z]+= ").expect("Regex not valid!");
        let re_equal = Regex::new("%[a-zA-Z]+%[a-zA-Z]+==").expect("Regex not valid!");
        let set_match: Vec<&str> = re_set.find_iter(&src).map(|mat| mat.as_str()).collect();
        let space_match: Vec<&str> = re_space.find_iter(&src).map(|mat| mat.as_str()).collect();
        let equal_match: Vec<&str> = re_equal.find_iter(&src).map(|mat| mat.as_str()).collect();
        
        // Extract the proper variable strings based on the structure of the obfuscation.
        let set_str: &str = set_match[0].split(" ").collect::<Vec<&str>>()[1];
        let space_str: &str = space_match[0].split(" ").collect::<Vec<&str>>()[1];
        let eq_str: &str = equal_match[0].split("%").collect::<Vec<&str>>()[2];
        self.set_str = set_str[0..set_str.len()-4].to_string();
        self.space_str = space_str[0..space_str.len()-2].to_string();
        self.eq_str = eq_str[0..eq_str.len()-2].to_string();

        // Reverse engineer the obfuscated alphabet and build a cleartext charset.
        self.reverse_alphabet(&src);

        // Deobfuscate the source code using the reverse-engineered obfuscation alphabet.
        self.deobfuscate(src);

        self.initialized = true;
    }

    /// Writes the deobfuscated source of a pre-initialized BatchDeobfuscator to a file, and returns a string containing the name of that file.<br><br>
    /// Output filename defaults to *deobfuscated.bat* when **None** is passed into the parameter.<br><br>
    /// **This method panics if file creation/writing fails.**
    pub fn write_deobfuscated_script(&self, file_name: Option<String>) -> String {

        if !self.initialized { panic!("Deobfuscator must first be initialized!"); };

        let handle_name: String = file_name.unwrap_or(String::from("deobfuscated.bat"));
        let handle_clone: String = handle_name.clone();

        let mut file = File::create(handle_clone.as_str()).expect("Failed to create file!");
        file.write_all(self.cleaned_code.as_bytes()).expect("Failed writing to file!");

        handle_name
    }

    /// Reverse-engineers an obfuscated alphabet using known patterns in the obfuscated source code.
    fn reverse_alphabet(&mut self, src: &str) {

        let re = Regex::new(r"[a-zA-Z]+%[a-zA-Z]+%.{1}\n").expect("Regex pattern invalid!");

        let matches: Vec<&str> = re.find_iter(src).map(|mat| mat.as_str()).collect();

        for mtch in matches {
            let chr: String = format!("{}", mtch.chars().nth(mtch.len()-2).unwrap());

            let name: String = String::from(mtch.split("%").collect::<Vec<&str>>()[0]);

            self.alphabet.insert(name, chr);
        };

        let re2 = Regex::new(r"[a-zA-Z]+%[a-zA-Z]+%.{2}\n").expect("Regex pattern invalid!");
        let matches: Vec<&str> = re2.find_iter(src).map(|mat| mat.as_str()).collect();

        for mtch in matches {
            let blob: String = format!(
                "{}{}",
                mtch.chars().nth(mtch.len()-3).unwrap(),
                mtch.chars().nth(mtch.len()-2).unwrap()
            );

            let name: String = String::from(mtch.split("%").collect::<Vec<&str>>()[0]);

            self.alphabet.insert(name, blob);
        };

        let re3 = Regex::new(r"%[a-zA-Z]+%%[a-zA-Z]+%[a-zA-Z]+%[a-zA-Z]+%.{3,}\n").expect("Regex pattern invalid!");
        let matches: Vec<&str> = re3.find_iter(src).map(|mat| mat.as_str()).collect();

        for mtch in matches {
            match mtch.find(&self.eq_str) {
                Some(index) => {
                    let name_haystack: Vec<&str> = mtch.split("%").collect();
                    let name: String = String::from(name_haystack[name_haystack.len()-3]);
                    let blob: &str = &mtch[(index+self.eq_str.len()+1)..];
                    println!("name {:#?}", mtch.split("%").collect::<Vec<&str>>());
                    self.alphabet.insert(name, blob.to_string());
                },
                None => {
                    continue;
                },
            };
        };
    }

    /// Deobfuscates a sample of obfuscated batch commands using a reverse-engineered obfuscation alphabet.
    fn deobfuscate(&mut self, src: String) {

        let src: Vec<&str> = src.split("\n").collect();

        // Iterate over the remaining obfuscated text and map the obfuscated strings to cleartext characters.
        let mut cleaned_chars: Vec<String> = Vec::new();
        for line in src {

            if line == "\n" || line == "\r\n" || line == "" { continue; };

            if line.contains(&self.set_str) || line.contains(&self.space_str) || line.contains(&self.eq_str) {
                continue;
            };

            if line.contains(":: VGhpcyBmaWxlIHdhcyBvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I=") 
            || line.contains(":: VGhpcyBmaWxlIGNhbiBiZSBwcm9ncmFtYXRpY2FsbHkgZGVvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I="){
                continue;
            };

            let code: Vec<&str> = line.split("%").collect();

            for blob in code {
                if let Some(chr) = self.alphabet.get(&blob.to_string()) {
                    cleaned_chars.push(chr.to_string());
                }else {
                    for c in self.alphabet.keys() {
                        if blob.contains(c.as_str()) { continue };
                    };

                    if !blob.contains(" ") && blob != "" && blob != "\r" {
                        let mut skip: bool = false;
                        for character in blob.chars() {
                            if CharSet::BadChars.values().contains(&character) {
                                skip = true;
                            };
                        };
                        if !skip { cleaned_chars.push(String::from("%")); };
                        for character in blob.chars() {
                            cleaned_chars.push(character.to_string());
                        };
                        if !skip { cleaned_chars.push(String::from("%")); };
                    }else {
                        for character in blob.chars() {
                            cleaned_chars.push(character.to_string());
                        };
                    };
                };
            };

            cleaned_chars.push(String::from("\n"));
        };

        // Clean up the last remaining artifacts of the obfuscation.
        cleaned_chars = cleaned_chars.join("").split("\n").map(|chr| chr.to_string()).collect();

        // Reassemble the cleartext code and finalize the initialization.
        self.cleaned_code = cleaned_chars.join("\n").trim_end().to_string();
    }
}