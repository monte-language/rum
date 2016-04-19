extern crate bincode;
extern crate docopt;
extern crate num;
extern crate rustc_serialize;
mod mast;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use docopt::Docopt;
use mast::MastReader;


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
            BufReader::new(
                Box::new(io::stdin()) as Box<Read>
            )
        },
        p@_ => {
            BufReader::new(
                Box::new(File::open(&p).unwrap()) as Box<Read>
            )
        },
    };

    // let mast_reader = BufReader::new(&args.arg_file);
    mast_reader.check_magic_numbers().unwrap();
    mast_reader.execute();
}
