extern crate docopt;
extern crate rustc_serialize;
mod parser;

use std::error::Error;

use docopt::Docopt;


const USAGE: &'static str = "
monstracite

Usage: monstracite [--file=<file>]
       monstracite (-h | --help)

Options:
    -f <file>, --file=<file>   Monte AST input [default: stdin]
    -h  --help  Show this information
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_file: String,
}

fn main() {
    let mut args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    parser::parse_file(&mut args.flag_file);

}
