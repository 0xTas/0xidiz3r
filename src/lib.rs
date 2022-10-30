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


use std::{
    time::Duration,
    io::{self, Write}, 
    thread::sleep,
};
pub mod batch;


/* General Utility Functions */

/// Prompts the user for input and returns it as a String.<br><br>
/// The *prompt: &str* parameter is printed without a trailing newline by default. Supply your own if desired.
pub fn input(prompt: &str) -> String {
    let mut user_input: String = String::new();

    print!("{}", prompt);
    io::stdout().flush().expect("Write to console failed!");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Read from stdin failed!");

    user_input
}

/// Sleeps for the given time in milliseconds.
pub fn wait(dur: u64) {
    let dur: Duration = Duration::from_millis(dur);
    sleep(dur);
}