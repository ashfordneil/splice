#[macro_use]
extern crate clap;
extern crate regex;

use std::io::BufRead;

mod setup;
use setup::Options;

fn main() {
    let mut options = Options::setup();

    // storage for each individual line in the input file
    let mut buffer = String::new();
    let mut depth: u32 = 0;
    'main: loop {
        // read line of input, stop if we've run out
        buffer.clear();
        match options.input().read_line(&mut buffer).unwrap() {
            0 => break 'main,
            _ => {}
        };

        if options.start().is_match(&buffer) {
            if depth < 1 {
                depth += 1;
            }
        }

        if depth > 0 {
            print!("{}", buffer);
        }

        if options.stop().is_match(&buffer) && depth > 0 {
            depth -= 1;

            if depth == 0 && !options.repeated() {
                break 'main;
            }
        }

    }
}
