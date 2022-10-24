### An attempt at reversible batch & powershell obfuscation using techniques inspired by John Hammond.<br>

Batch technique: https://www.youtube.com/watch?v=0RADvfJysuA<br>

Powershell technique (todo!()): https://www.youtube.com/watch?v=WJlqQYyzGi8<br>

### Disclaimer

**This program is for educational purposes only. This obfuscation is fairly trivial to reverse engineer. Don't be malicious.**<br>

### Usage<br>

`cargo run`<br>

The program will prompt you for a batch command or path to a batch script.<br>

Then, a script named "obfuscated.bat" will be created in the current directory.<br>

The contents of .\obfuscated.bat is functionally equivalent to your source command/script.

Then, the program will ask you for a path to an obfuscated script. It will attempt to deobfuscate it and dump the output to .\deobfuscated.bat.