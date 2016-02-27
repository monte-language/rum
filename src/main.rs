extern crate docopt;
extern crate num;
extern crate rustc_serialize;
mod parser;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use docopt::Docopt;
use parser::MastReader;


const USAGE: &'static str = "
rum

Usage: rum [<file>]
       rum (-h | --help)

Options:
    <file>      Optional: Input Mast File,
                else read from stdin
    -h --help   Show this information
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
}

// Appropriately handle the given arguments and begin parsing
fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let mut mast_reader = match &args.arg_file as &str {
        "" => {
            let b: Box<Read> = Box::new(io::stdin());
            let a = BufReader::new(b);
            a
        },
        p@_ => {
            let b: Box<Read> = Box::new(File::open(&p).unwrap());
            let a = BufReader::new(b);
            a
        },
    };

    // let mast_reader = BufReader::new(&args.arg_file);
    mast_reader.check_magic_numbers().unwrap();
    mast_reader.execute();
}
