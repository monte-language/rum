extern crate docopt;
extern crate rustc_serialize;
mod parser;

use std::error::Error;
use std::fs::File;

use docopt::Docopt;


const USAGE: &'static str = "
monstracite

Usage:
    monstracite <file>
    monstracite (-h | --help)

Options:
    <file>      Monte source to compile
    -h  --help  Show this information
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
}

fn main() {
    let mut args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let source = match File::open(&mut args.arg_file) {
        Ok(r) => r,
        // This will display if file doesn't exist, etc
        Err(e) => panic!("{:}", e),
    };

    parser::parse_file(&source);

}
