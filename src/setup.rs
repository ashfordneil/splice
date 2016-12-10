use std::io;
use std::fmt;
use regex;

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use clap::{Arg, App};

/// A combination of the various error types that can arise during setup.
#[derive(Debug)]
pub enum Error {
    /// IO Errors.
    Io(io::Error),
    /// Regex Errors.
    Regex(regex::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref error) => write!(f, "{}", error),
            Error::Regex(ref error) => write!(f, "{}", error),
        }
    }
}

/// Summary of the options yielded by parsing the command line arguments.
pub struct Options {
    /// The regex pattern that opens the filter.
    start: Regex,
    /// The regex pattern that closes the filter.
    stop: Regex,
    /// A buffered reader of the input file.
    input: BufReader<Box<io::Read>>,
    // /// Is the program to operate in recursive mode.
    // recursive: bool,
}

impl Options {
    /// Parses command line arguments, and returns the Options struct (in a Result wrapper).
    pub fn setup() -> Result<Options, Error> {
        let matches = App::new("splice")
            .version("0.2.0")
            .author("Neil Ashford <ashfordneil0@gmail.com>")
            .about("A flexible, regex based command line filter")
            .arg(Arg::with_name("open_regex")
                .required(true)
                .help("The OPEN regex. Matches the (optionally recursive) entrance to the \
                       splice."))
            .arg(Arg::with_name("close_regex")
                .required(true)
                .help("The CLOSE regex. Matches the (optionally recursive) exit to the splice."))
            .arg(Arg::with_name("input")
                .required(false)
                .value_name("FILE")
                .help("The file to splice. Defaults to stdin."))
            .arg(Arg::with_name("recursive")
                .required(false)
                .short("r")
                .long("recursive")
                .help("Applies splice recursively. Each match of the OPEn regex pushes a new \
                       layer onto a stack, and each match of the CLOSE regex pops the previous \
                       layer off of the stack."))
            .get_matches();

        let start = try!(Regex::new(matches.value_of("open_regex").unwrap()).map_err(Error::Regex));
        let stop = try!(Regex::new(matches.value_of("close_regex").unwrap()).map_err(Error::Regex));

        let input: BufReader<Box<io::Read>> = match matches.value_of("input") {
            Some(filename) => {
                let file = try!(File::open(filename).map_err(Error::Io));
                BufReader::new(Box::new(file))
            }
            None => {
                let file = io::stdin();
                BufReader::new(Box::new(file))
            }
        };

        // let recursive = match matches.value_of("recursive").unwrap();

        Ok(Options {
            start: start,
            stop: stop,
            input: input,
            // recursive: recursive,
        })
    }

    /// Return the start regex field of the options.
    pub fn start(&self) -> &Regex {
        return &self.start;
    }

    pub fn stop(&self) -> &Regex {
        return &self.stop;
    }

    pub fn input(&mut self) -> &mut BufReader<Box<io::Read>> {
        return &mut self.input;
    }
}
