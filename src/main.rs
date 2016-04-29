extern crate bincode;
extern crate docopt;
extern crate num;
extern crate rustc_serialize;
mod load_mast;
mod nodes;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use docopt::Docopt;


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

    let mast_reader = match &args.arg_file as &str {
        "" => {
            let i = Box::new(io::stdin()) as Box<Read>;
            Box::new(BufReader::new(i))
        },
        p@_ => {
            let i = Box::new(File::open(&p).unwrap()) as Box<Read>;
            Box::new(BufReader::new(i))
        },
    };

    load_mast::load(mast_reader);
}
