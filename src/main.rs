extern crate docopt;
extern crate num;
extern crate rustc_serialize;
mod parser;

use std::fs::File;

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

    parser::parse_file(&args.arg_file);
}
