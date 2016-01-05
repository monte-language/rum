extern crate docopt;
extern crate rustc_serialize;
mod parser;

use std::fs::File;

use docopt::Docopt;


const USAGE: &'static str = "
monstracite

Usage: monstracite [<file>] [(-o <output> | --output <output>)]
       monstracite (-h | --help)

Options:
    -o --output <output>    File Path to output executable [default: a.out]
    -h --help               Show this information
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_output: String,
    arg_file: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    parser::parse_file(&args.arg_file);
    File::create(&args.flag_output).unwrap();
}
