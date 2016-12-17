use std::io;

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use clap::{Arg, App};

/// Summary of the options yielded by parsing the command line arguments.
pub struct Options {
    /// The regex pattern that opens the filter.
    start: Regex,
    /// The regex pattern that closes the filter.
    stop: Regex,
    /// A buffered reader of the input file.
    input: BufReader<Box<io::Read>>,
    /// Is the program to operate in repeated mode.
    repeated: bool,
}

impl Options {
    /// Parses command line arguments, and returns the Options struct.
    pub fn setup() -> Options {
        let matches = App::new("splice")
            .version("0.2.0")
            .author("Neil Ashford <ashfordneil0@gmail.com>")
            .about("A flexible, regex based command line filter")
            .arg(Arg::with_name("open_regex")
                .required(true)
                .validator(is_valid_regex)
                .help("The OPEN regex. Matches the (optionally repeated) entrance to the \
                       splice."))
            .arg(Arg::with_name("close_regex")
                .required(true)
                .validator(is_valid_regex)
                .help("The CLOSE regex. Matches the (optionally repeated) exit to the splice."))
            .arg(Arg::with_name("input")
                .validator(is_valid_filename)
                .value_name("FILE")
                .help("The file to splice. Defaults to stdin."))
            .arg(Arg::with_name("loop")
                .short("l")
                .long("looped")
                .help("Applies splice repeatedly to the input file. Defaults to false."))
            .arg(Arg::with_name("don't loop")
                 .short("L")
                 .long("not-looped")
                 .help("Only applies splice to the input file once.")
                 .overrides_with("loop"))
            .get_matches();

        let start = Regex::new(matches.value_of("open_regex").unwrap()).unwrap();
        let stop = Regex::new(matches.value_of("close_regex").unwrap()).unwrap();

        let input: BufReader<Box<io::Read>> = match matches.value_of("input") {
            Some(filename) => {
                let file = File::open(filename).unwrap();
                BufReader::new(Box::new(file))
            }
            None => {
                let file = io::stdin();
                BufReader::new(Box::new(file))
            }
        };

        let repeated = matches.is_present("loop");

        Options {
            start: start,
            stop: stop,
            input: input,
            repeated: repeated,
        }
    }

    /// Return the start regex field of the options.
    pub fn start(&self) -> &Regex {
        return &self.start;
    }

    /// Returns the stop regex field of the options.
    pub fn stop(&self) -> &Regex {
        return &self.stop;
    }

    /// Returns the input file of the options.
    pub fn input(&mut self) -> &mut BufReader<Box<io::Read>> {
        return &mut self.input;
    }

    /// Returns the repeated / loop field of the options.
    pub fn repeated(&self) -> bool {
        return self.repeated;
    }
}

/// Determines if regex is valid or not. Used as a validator for CLAP.
fn is_valid_regex(argument: String) -> Result<(), String> {
    match Regex::new(argument.as_str()) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

/// Determines if a file can be opened in read mode or not. TODO find a way to do this without
/// actually openning the file. Used as a validator for CLAP.
fn is_valid_filename(argument: String) -> Result<(), String> {
    match File::open(argument) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
