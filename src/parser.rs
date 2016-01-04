use std::io::prelude::*;

use std::fs::File;
use std::io::BufReader;

const MAGIC: &'static [u8; 10] = b"Mont\xe0MAST\x00";

fn check_magic_numbers(nums: &[u8]) -> bool {
    let mut are_we_good: bool = true;

    for i in 0..10 {
        if nums[i] != MAGIC[i] {
            are_we_good = false;
            break;
        }
    }

    are_we_good
}

pub fn parse_file(source: &File) {
    let buf = BufReader::new(source);

    // File type check (MontyMASTnull)
    let mut numbers_buf = buf.take(10);

    let mut numbers: [u8; 10] = [0; 10];
    numbers_buf.read(&mut numbers).unwrap();

    match check_magic_numbers(&numbers) {
        true => {},
        false => panic!("Invalid File Type"),
    }

    println!("Nice brah, we got us a Monte AST");
}
