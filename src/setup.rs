use std::io;
use std::fmt;
use regex;

use std::io::BufReader;
use std::fs::File;
use regex::Regex;

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
}

impl Options {
    /// Parses command line arguments, and returns the Options struct (in a Result wrapper).
    pub fn setup() -> Result<Options, Error> {
        let matches = clap_app!(splice =>
            (version: "0.2.0")
            (author: "Neil Ashford <ashfordneil0@gmail.com>")
            (about: "A flexible, regex based command line filter")
            (@arg START_REGEX: +required "The regex to start the filter at")
            (@arg STOP_REGEX: +required "The regex to stop the filter at")
            (@arg INPUT: "The input file to read from (defaults to stdin)")
        )
            .get_matches();

        let start = try!(Regex::new(matches.value_of("START_REGEX").unwrap())
            .map_err(Error::Regex));
        let stop = try!(Regex::new(matches.value_of("STOP_REGEX").unwrap()).map_err(Error::Regex));

        let input: BufReader<Box<io::Read>> = match matches.value_of("INPUT") {
            Some(filename) => {
                let file = try!(File::open(filename).map_err(Error::Io));
                BufReader::new(Box::new(file))
            }
            None => {
                let file = io::stdin();
                BufReader::new(Box::new(file))
            }
        };

        Ok(Options {
            start: start,
            stop: stop,
            input: input,
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
