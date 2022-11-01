### An attempt at reversible batch & powershell obfuscation using techniques inspired by John Hammond.<br>

Batch technique: https://www.youtube.com/watch?v=0RADvfJysuA<br>

Powershell technique (todo!()): https://www.youtube.com/watch?v=WJlqQYyzGi8<br>

## Background (Batch)

**Obfuscation:**<br>

This is an implementation of an interesting Batch obfuscation technique which uses variables to obfuscate Batch commands.<br>
The basic idea is that you can obfuscate many keywords and characters in Batch using nonsensical variable names, and they will still work.<br>
This program builds an obfuscated alphabet using random variable names, and uses those random variables to build obfuscated commands.<br>
In my testing, this particular technique is trivial to unravel as a human analyst, but may still avoid static detection in some scenarios.<br>

**This program makes no guarantees about preserving the functionality of the obfuscated scripts it produces.**<br><br>

*Most things seem to work, however there are notable limitations:*<br><br>
**Variables:** User-defined or environment variables cannot be effectively obfuscated using this technique.
This is because, while Batch **can** technically double-nest variable definitions and usages **as long as** an obfuscation variable is defined on the entire line,
the act of defining that line as an obfuscated variable will still require printing the cleartext un-obfuscated version of that line into the output script, effectively canceling the obfuscation effect.<br>

**Function Labels:** Batch function labels `:EXAMPLE`, can also not be obfuscated letter-by-letter, but the `goto :EXAMPLE` commands can be.
I have chosen to leave labels alone for now, as obfuscating the entire label with a single variable would also not really accomplish much.<br><br>

If you come across keywords or patterns which fail to execute in an obfuscated state, please open an issue and I will investigate how to best handle similar cases in the program.<br>
Alternatively, if you wish to make those fixes/improvements yourself, *pull requests are welcomed*.<br><br>

---

**Deobfuscation:**<br>

I was completely in the dark when implementing the deobfuscation, but I have gotten it to a point that I feel is mostly reliable.<br>
Using Regex patterns and assumptions made about the structure of the obfuscation, I have managed to get the deobfuscator to perfectly restore all of my test input scripts to their original forms.<br><br>

**This program makes no guarantees about the fidelity or accuracy of the deobfuscated scripts it produces.**<br><br>

As long as the input script was obfuscated using the above technique, where the obfuscated variable names contain only letters, 
and as long as the obfuscated alphabet is mostly intact (each obfuscated variable used needs a matching definition statement), the deobfuscation **should** work.<br>

That being said, I don't do much with Batch, and didn't have a wide variety of scripts to test with.<br>
So, same as above, I am open to issues or pull requests regarding the performance of this deobfuscator.<br>

---


## Disclaimer

**This program is for educational purposes only. This obfuscation technique is trivial to reverse engineer.**<br>
**Don't be malicious.**

---

## Usage <br><br>

### As a Library: <br>

You can use *0xidiz3r* as a library if you wish to write your own abstractions over the (currently minimal) API.<br><br>
**Obfuscation:**<br>
```rust
// Bring the obfuscator struct into scope:
use oxidizer::batch::obfuscator::BatchObfuscator

// Create an empty instance of a batch obfuscator:
let obfuscator = BatchObfuscator::new();

// Initialize the obfuscator with some source commands:
let poc_cmd = "start C:/Windows/System32/calc.exe";
obfuscator.initialize(None, None, poc_cmd.to_string());

// The initialization step takes some source commands and automatically obfuscates them.
// You can then write that obfuscated output to a file of your choice:
let filename = String::from("obfuscated.bat");
obfuscator.write_obfuscated_script(Some(filename));

// You could also print to stdout if necessary, though it can be spammy and copy/pasting might be unreliable:
println!("{:#?}", obfuscator.obfuscated_code);
```
<br><br>
**Deobfuscation:**<br>
```rust
// Bring the deobfuscator struct into scope:
use oxidizer::batch::deobfuscator::BatchDeobfuscator;

// Create an empty instance of a batch deobfuscator:
let deobfuscator = BatchDeobfuscator::new();

// Initialize the deobfuscator with some obfuscated source (alphabet + commands):
let source = fs::read_to_string("obfuscated.bat").unwrap();
deobfuscator.initialize(source);

// The initialization step takes some obfuscated source commands and attempts to deobfuscate them.
// If successful, you can write the deobfuscated output to a file:
deobfuscator.write_deobfuscated_script(None); // Using "None" causes the method to use a default filename: "deobfuscated.bat".

// Or alternatively, to stdout:
println!("{:#?}", deobfuscator.cleaned_code);
```

---

### As a CLI Utility: <br>
The following assumes that you have either downloaded a release binary or cloned the repo and built it using `cargo build --release`.<br><br>

A simple CLI tool is provided as an abstraction over the API for your convenience:<br>

**Windows:**<br>
```powershell
# To obfuscate a source command from stdin (with -e to disable command echoing):
.\0xidiz3r.exe "start C:/Windows/System32/calc.exe" -e

# To obfuscate a source file with the default configuration:
.\0xidiz3r.exe input.bat

# To obfuscate a source file, adding "@echo off" and writing to a custom output file:
.\0xidiz3r.exe input.bat -e -o output.bat

# To deobfuscate a source file:
.\0xidiz3r.exe -d output.bat
```
<br>

**Linux/MacOS:**<br>
```bash
# To obfuscate a source command from stdin (with -e to disable command echoing):
./0xidiz3r "start C:/Windows/System32/calc.exe" -e

# To obfuscate a source file with the default configuration:
./0xidiz3r input.bat

# To obfuscate a source file, adding "@echo off" and writing to a custom output file:
./0xidiz3r input.bat -e -o output.bat

# To deobfuscate a source file with a custom output file-name:
./0xidiz3r -d output.bat -o cleaned.bat
```

---

## Future Plans

- Improve batch obfuscation technique (recursive reuse? ascii exit codes? additional modes?)
- Improve batch deobfuscation technique to match
- Introduce Powershell path obfuscation module
- Introduce Powershell path deobfuscation module (accepts script with obfuscated binary paths and attempts to deobfuscate them)