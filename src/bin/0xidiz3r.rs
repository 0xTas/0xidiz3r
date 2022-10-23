use oxidizer::input;
use oxidizer::batch::obfuscator::BatchObfuscator;
use std::fs;


fn main() {
    // let poc: &str = "start C:/WINDOWS/System32/calc.exe";

    let mut obfuscator: BatchObfuscator = BatchObfuscator::new();

    let target: String = input("Enter Batch Command or Path to File ~> ");

    let file_check = target.clone();
    if let Ok(contents) = fs::read_to_string(file_check.as_str().trim_end()) {
        obfuscator.initialize(None, None, contents.as_str().trim_end());
        let path = obfuscator.write_obfuscated_script(None);
        println!("\nDumped obfuscated output to file: {}\nObfuscation Complete.", path);
        
    }else {
        obfuscator.initialize(None, None, target.as_str().trim_end());
        let path = obfuscator.write_obfuscated_script(None);
        println!("\nDumped obfuscated output to file: {}\nObfuscation Complete.", path);
    };
}