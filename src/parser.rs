use std::io::prelude::*;

use std::env::{temp_dir};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

const MAGIC: &'static [u8; 10] = b"Mont\xe0MAST\x00";

type BufMASTReader<R> = BufReader<R>;

trait Mast {
    fn check_magic_numbers(&mut self) -> Result<(), &str>;
}

impl<R: Read> Mast for BufMASTReader<R> {
    fn check_magic_numbers(&mut self) -> Result<(), &str> {
        let mut nums = [0u8; 10];
        self.take(10).read(&mut nums).unwrap();
        for i in 0..10 {
            if nums[i] != MAGIC[i] {
                return Err("Filetype is not Monte AST")
            }
        }
        Ok(())
    }
}

pub fn parse_file(source_path: &str) {
    let mut mast_reader = match source_path {
        e@"" => {
            // Sadly, since stdin cannot be a File, dump stdin to a tmp file
            // then read it. It's lame...but oh well
            let path = Path::join(temp_dir().as_path(), "stdin_mast");
            let mut tmp_file = File::create(path).unwrap();
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf).unwrap();
            tmp_file.write_all(&mut buf).unwrap();
            let path = Path::join(temp_dir().as_path(), "stdin_mast");
            BufMASTReader::new(File::open(&path).unwrap())
        },
        f@_ => BufMASTReader::new(File::open(&f).unwrap()),
    };

    match mast_reader.check_magic_numbers() {
        Err(e) => println!("{:}", e),
        Ok(_) => println!("MAST Found"),
    }
}
