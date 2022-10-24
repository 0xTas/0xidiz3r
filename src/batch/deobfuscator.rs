use std::{fs::File, io::Write, collections::HashMap};
use regex::Regex;


#[derive(Debug)]
pub struct BatchDeobfuscator {
    pub set_str: String,
    pub space_str: String,
    pub eq_str: String,
    pub alphabet: HashMap<String, char>,
    pub obfuscated_source: String,
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
            obfuscated_source: String::new(),
            cleaned_code: String::new(),
            initialized: false,
        }
    }

    // WHY THE FUCK DOES THIS NOT WORK IF A SINGLE BYTE OF THE OBFUSCATED FILE HAS BEEN ALTERED IN ANY WAY WTF
    pub fn initialize(&mut self, src: String) {

        self.obfuscated_source = src;

        // Pattern matching to identify set, space, and equals variables.
        let re_set = Regex::new("set [a-zA-Z]+=set").expect("Regex not valid!");
        let re_space = Regex::new("%[a-zA-Z]+% [a-zA-Z]+= ").expect("Regex not valid!");
        let re_equal = Regex::new("%[a-zA-Z]+%[a-zA-Z]+==").expect("Regex not valid!");
        let set_match: Vec<&str> = re_set.find_iter(&self.obfuscated_source).map(|mat| mat.as_str()).collect();
        let space_match: Vec<&str> = re_space.find_iter(&self.obfuscated_source).map(|mat| mat.as_str()).collect();
        let equal_match: Vec<&str> = re_equal.find_iter(&self.obfuscated_source).map(|mat| mat.as_str()).collect();
        
        // Extract the proper variable strings based on the structure of the obfuscation.
        let set_str: &str = set_match[0].split(" ").collect::<Vec<&str>>()[1];
        let space_str: &str = space_match[0].split(" ").collect::<Vec<&str>>()[1];
        let eq_str: &str = equal_match[0].split("%").collect::<Vec<&str>>()[2];
        self.set_str = set_str[0..set_str.len()-4].to_string();
        self.space_str = space_str[0..space_str.len()-2].to_string();
        self.eq_str = eq_str[0..eq_str.len()-2].to_string();

        // Reverse engineer the obfuscated alphabet and build a cleartext charset.
        self.reverse_alphabet();

        // Pull out any boilerplate variable definition lines
        let re_set_marker = Regex::new(format!("\n%{}%.?*\n", self.set_str).as_str()).expect("Regex not valid!");
        let actual_src: Vec<&str> = re_set_marker.split(&self.obfuscated_source).collect();

        // Iterate over the remaining obfuscated text and map the obfuscated strings to cleartext characters.
        let mut cleaned_chars: Vec<String> = Vec::new();
        for var in actual_src.join("\n").split("%") {

            if var.contains(&self.set_str) || var.contains(&self.space_str) || var.contains(&self.eq_str) {
                continue;
            };

            if var.starts_with("::") {
                continue;
            };

            if let Some(chr) = self.alphabet.get(&var.to_string()) {
                cleaned_chars.push(chr.to_string());
            }else {
                for c in self.alphabet.keys() {
                    if var.contains(c.as_str()) { continue };
                };

                for character in var.chars() {
                    cleaned_chars.push(character.to_string());
                };
            };
        };

        // Clean up the last remaining artifacts of the obfuscation.
        cleaned_chars = cleaned_chars.join("").split("\n").map(|chr| chr.to_string()).collect();
        let mut offset = 0;
        for i in 0..44 {
            cleaned_chars.remove((i-offset).try_into().unwrap());
            offset += 1;
        };

        // Reassemble the cleartext code and finalize the initialization.
        self.cleaned_code = cleaned_chars.join("\n");
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


    fn reverse_alphabet(&mut self) {

        let re = Regex::new(r"[a-zA-Z]+%[a-zA-Z]+%.{1}\n").expect("Regex not valid!");

        let matches: Vec<&str> = re.find_iter(&self.obfuscated_source).map(|mat| mat.as_str()).collect();

        for mtch in matches {
            let chr: char = mtch.chars().nth(mtch.len()-2).unwrap();

            let name: String = String::from(mtch.split("%").collect::<Vec<&str>>()[0]);

            self.alphabet.insert(name, chr);
        };
    }
}