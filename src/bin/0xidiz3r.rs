use oxidizer::input;
use oxidizer::batch::obfuscator::BatchObfuscator;
use oxidizer::batch::deobfuscator::BatchDeobfuscator;
use std::fs;


fn main() {
    // let poc: &str = "start C:/WINDOWS/System32/calc.exe";

    let mode: String = input("Default mode is obfuscate. Switch to deobfuscate mode? [Y/N] ~> ");

    if mode.to_lowercase().contains("y") {
            let mut deobfuscator = BatchDeobfuscator::new();

            let target: String = input("Enter Path to Obfuscated File ~> ");

            let file_check = target.clone();
            if let Ok(contents) = fs::read_to_string(file_check.as_str().trim_end()) {
                deobfuscator.initialize(contents);

                let path: String = deobfuscator.write_deobfuscated_script(None);
                println!("\nDumped deobfuscated output to file: {}\nDeobfuscation Complete.", path);
            }else {
                panic!("File not found!");
            };
    }else {

        let mut obfuscator: BatchObfuscator = BatchObfuscator::new();

        let target: String = input("Enter Batch Command or Path to File ~> ");

        let file_check = target.clone();
        if let Ok(contents) = fs::read_to_string(file_check.as_str().trim_end()) {
            if contents.contains("echo") {
                println!("\n[!]--> INFO: By default, \"@echo off\" is inserted to aid the obfuscation.");
                let cont: String = input("Would you like to preserve the script's echo functionality instead? [Y/N] ~> ");

                if cont.to_lowercase().contains("y") {
                    obfuscator.enable_echo();
                };
            };

            obfuscator.initialize(None, None, contents.trim_end().to_string());
            let path = obfuscator.write_obfuscated_script(None);
            println!("\nDumped obfuscated output to file: {}\nObfuscation Complete.", path);
            
        }else {
            obfuscator.initialize(None, None, target.trim_end().to_string());
            let path = obfuscator.write_obfuscated_script(None);
            println!("\nDumped obfuscated output to file: {}\nObfuscation Complete.", path);
        };
    };


    

}