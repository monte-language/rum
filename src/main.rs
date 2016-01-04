extern crate docopt;
extern crate rustc_serialize;

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
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
}
