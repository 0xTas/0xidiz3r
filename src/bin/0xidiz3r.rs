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


use clap::Parser;
use std::{
    fs,
    process::exit
};
use oxidizer::{
    wait,
    batch::{
        obfuscator::BatchObfuscator,
        deobfuscator::BatchDeobfuscator,
    },
};


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {

    /// Input File or Command
    input: String,

    /// Use deobfuscation mode
    #[arg(short, long, default_value_t = false)]
    deobfuscate: bool,

    /// Add "@echo off" to the output script to avoid echoing cleartext commands
    #[arg(short, long, default_value_t = false)]
    echo_off: bool,

    /// Custom name for the output file
    #[arg(short, long)]
    output_file: Option<String>,

    /// Minimum obfuscated variable length
    #[arg(long)]
    min: Option<u32>,

    /// Maximum obfuscated variable length
    #[arg(long)]
    max: Option<u32>
}


fn main() {
    
    let mut args = Args::parse();

    if let Some(min_value) = args.min {
        if let Some(max_value) = args.max {
            if min_value >= max_value {
                args.max = Some(args.min.unwrap() * 2);
            };
        };
    };

    if args.deobfuscate {
        let mut deobfuscator: BatchDeobfuscator = BatchDeobfuscator::new();

        if let Ok(contents) = fs::read_to_string(args.input.trim_end()) {
            deobfuscator.initialize(contents);
        }else {
            println!("\nError! File not found.");
            println!("Please provide a valid path to an obfuscated file!");
            wait(4200);
            exit(1);
        };

        let path: String = deobfuscator.write_deobfuscated_script(args.output_file);
        println!("\nDumped deobfuscated output to file: {}\nDeobfuscation Complete.", path);

        exit(0);
    }else {
        let mut obfuscator: BatchObfuscator = BatchObfuscator::new();
        if args.echo_off {
            obfuscator.dont_echo();
        };

        if let Ok(contents) = fs::read_to_string(args.input.trim_end()) {
            obfuscator.initialize(args.min, args.max, contents);

            let path: String = obfuscator.write_obfuscated_script(args.output_file);
            println!("\nDumped obfuscated output to file: {}\nObfuscation Complete.", path);
        }else {
            obfuscator.initialize(args.min, args.max, args.input);

            let path: String = obfuscator.write_obfuscated_script(args.output_file);
            println!("\nDumped obfuscated output to file: {}\nObfuscation Complete.", path);
        };

        exit(0);
    };
}