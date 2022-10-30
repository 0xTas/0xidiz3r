use oxidizer::wait;
use oxidizer::batch::obfuscator::BatchObfuscator;
use oxidizer::batch::deobfuscator::BatchDeobfuscator;
use clap::Parser;
use std::{fs, process::exit};


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {

    /// Input File or Command
    input: String,

    /// Use deobfuscation mode
    #[arg(short, long, default_value_t = false)]
    deobfuscate: bool,

    /// Don't add "@echo off" to the output script.
    #[arg(long, default_value_t = true)]
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
        let mut obfuscator :BatchObfuscator = BatchObfuscator::new();
        if !args.echo_off {
            obfuscator.enable_echo();
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