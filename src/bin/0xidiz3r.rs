use oxidizer::{input, define_batch_variable};
use oxidizer::batch::BatchObfuscator;
use std::fs::File;
use std::io::Write;


// TODO: Handle batch command length limit of 8191 bytes with dynamic payload-length adjustments?
fn create_obfuscated_batch(src: &str) {
    let obfuscator: BatchObfuscator = BatchObfuscator::new(None, None);

    let mut commands: Vec<String> = Vec::new();
    let mut execute: Vec<String> = Vec::new(); 

    commands.push(String::from(":: This file was obfuscated via https://github.com/0xTas/0xidiz3r "));
    commands.push(String::from(":: This file can be programatically deobfuscated (soon™) via https://github.com/0xTas/0xidiz3r "));
    commands.push(String::from("@echo off"));
    commands.push(format!("set {}=set", obfuscator.set_str));
    commands.push(format!("%{}% {}= ", obfuscator.set_str, obfuscator.space_str));
    commands.push(format!("%{}%%{}%{}==", obfuscator.set_str, obfuscator.space_str, obfuscator.eq_str));

    for chr in src.chars() {
        let varname: &String = obfuscator.alphabet.get(&chr).expect("Key not in alphabet!");

        commands.push(define_batch_variable(format!("{}", varname.to_owned()), format!("{}", chr.to_owned()), &obfuscator));
        execute.push(format!("%{}%", varname.to_owned()));
    };

    let execute_string: String = execute.join("");
    commands.push(execute_string);
    commands.push(String::from(":: This file was obfuscated via https://github.com/0xTas/0xidiz3r "));
    commands.push(String::from(":: This file can be programatically deobfuscated (soon™) via https://github.com/0xTas/0xidiz3r "));

    let final_code: String = commands.join("\n");
    let mut file = File::create("obfuscated.bat").expect("Failed to create file!");
    file.write_all(final_code.as_bytes()).expect("Failed writing to file!");
}

fn main() {
    // let poc: &str = "start C:/WINDOWS/System32/calc.exe";

    let poc: String = input("Enter Batch Command ~> ");

    create_obfuscated_batch(poc.as_str().trim_end());
    println!("Obfuscation Complete.");
}