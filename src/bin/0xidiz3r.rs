use oxidizer::input;
use oxidizer::batch::BatchObfuscator;
use std::fs;


fn main() {
    // let poc: &str = "start C:/WINDOWS/System32/calc.exe";

    let mut obfuscator: BatchObfuscator = BatchObfuscator::new();

    let poc: String = input("Enter Batch Command or Path to File ~> ");

    let file_check = poc.clone();
    if let Ok(contents) = fs::read_to_string(file_check.as_str().trim_end()) {
        obfuscator.initialize(None, None, contents.as_str().trim_end());
        let path = obfuscator.write_obfuscated_script(None);
        println!("Obfuscation Complete.\n{}", path);
        
    }else {
        obfuscator.initialize(None, None, poc.as_str().trim_end());
        let path = obfuscator.write_obfuscated_script(None);
        println!("Obfuscation Complete.\n{}", path);
    };
}