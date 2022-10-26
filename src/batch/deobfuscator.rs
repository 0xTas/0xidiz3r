use std::{fs::File, io::Write, collections::HashMap};
use regex::Regex;


#[derive(Debug)]
pub struct BatchDeobfuscator {
    pub set_str: String,
    pub space_str: String,
    pub eq_str: String,
    pub alphabet: HashMap<String, char>,
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

        self.deobfuscate(src);

        self.initialized = true;
    }


    pub fn write_deobfuscated_script(&self, file_name: Option<String>) -> String {

        if !self.initialized { panic!("Deobfuscator must first be initialized!"); };

        let handle_name: String = file_name.unwrap_or(String::from("deobfuscated.bat"));
        let handle_clone: String = handle_name.clone();

        let mut file = File::create(handle_clone.as_str()).expect("Failed to create file!");
        file.write_all(self.cleaned_code.as_bytes()).expect("Failed writing to file!");

        handle_name
    }


    fn reverse_alphabet(&mut self, src: &str) {

        let re = Regex::new(r"[a-zA-Z]+%[a-zA-Z]+%.{1}\n").expect("Regex not valid!");

        let matches: Vec<&str> = re.find_iter(src).map(|mat| mat.as_str()).collect();

        for mtch in matches {
            let chr: char = mtch.chars().nth(mtch.len()-2).unwrap();

            let name: String = String::from(mtch.split("%").collect::<Vec<&str>>()[0]);

            self.alphabet.insert(name, chr);
        };
    }

    fn deobfuscate(&mut self, src: String) {

        // Pull out any boilerplate variable definition lines
        let re_set_marker = Regex::new(format!("\n%{}%.?*\n", self.set_str).as_str()).expect("Regex not valid!");
        // let src: Vec<&str> = re_set_marker.split(&src).collect();

        let src: Vec<&str> = src.split("\n").collect();

        // Iterate over the remaining obfuscated text and map the obfuscated strings to cleartext characters.
        let mut cleaned_chars: Vec<String> = Vec::new();
        for line in src {

            if line == "\n" || line == "\r\n" { continue; };

            if line.contains(&self.set_str) || line.contains(&self.space_str) || line.contains(&self.eq_str) {
                continue;
            };

            if line.contains(":: VGhpcyBmaWxlIHdhcyBvYmZ1c2NhdGVkIHZpYSBodHRwczovL2dpdGh1Yi5jb20vMHhUYXMvMHhpZGl6M3I=") 
            || line.contains(":: VGhpcyBmaWxlIGNhbiBiZSBwcm9ncmFtYXRpY2FsbHkgZGVvYmZ1c2NhdGVkIChzb29u4oSiKSB2aWEgaHR0cHM6Ly9naXRodWIuY29tLzB4VGFzLzB4aWRpejNy"){
                continue;
            };

            let lines: Vec<&str> = line.split("%").collect();

            for blob in lines {
                if let Some(chr) = self.alphabet.get(&blob.to_string()) {
                    cleaned_chars.push(chr.to_string());
                }else {
                    for c in self.alphabet.keys() {
                        if blob.contains(c.as_str()) { continue };
                    };

                    if !blob.contains(" ") && blob != "" && blob != "\r" {
                        cleaned_chars.push(String::from("%"));
                        for character in blob.chars() {
                            cleaned_chars.push(character.to_string());
                        };
                        cleaned_chars.push(String::from("%"));
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
        self.cleaned_code = cleaned_chars.join("\n");
    }
}