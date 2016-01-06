use std::io::prelude::*;

use std::env::temp_dir;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use num::Zero;
use num::bigint::{BigInt, BigUint, ToBigInt};
use num::traits::{FromPrimitive, ToPrimitive};


type BufMASTReader<R> = BufReader<R>;

const MAGIC: &'static [u8; 10] = b"Mont\xe0MAST\x00";

fn unpack_float(bytes: Vec<u8>, endian: bool) -> f64 {
    panic!("{:?}, {:?}\nNot yet implemented!", bytes, endian);
    // 0.0 as f64
}


trait Mast {
    fn check_magic_numbers(&mut self) -> Result<(), &str>;
    fn take_n_bytes(&mut self, n: usize) -> Vec<u8>;
    fn next_byte(&mut self) -> u8;
    fn next_bytes(&mut self, count: usize) -> Vec<u8>;
    fn next_double(&mut self) -> f64;
    fn next_varint(&mut self) -> BigInt;
    fn next_int(&mut self) -> u64;
    fn next_str(&mut self) -> String;
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

    fn take_n_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(n);
        let mut t = self.take(n.clone() as u64);
        t.read(bytes.as_mut()).unwrap();
        println!("Taking: {:?}", bytes);
        bytes
    }

    fn next_byte(&mut self) -> u8 {
        self.take_n_bytes(1).pop().unwrap()
    }


    fn next_bytes(&mut self, count: usize) -> Vec<u8> {
        self.take_n_bytes(count)
    }

    fn next_double(&mut self) -> f64 {
        let bytes = self.take_n_bytes(8);
        unpack_float(bytes, true)
    }

    fn next_varint(&mut self) -> BigInt {
        let mut shift: usize = 0;
        let mut bi: BigUint = BigUint::zero();
        let mut b: u8 = 0;
        let mut and_shift: usize = 0;
        loop {
            b = self.next_byte();
            and_shift = (b as usize & 0x7f) << shift;
            bi = bi | BigUint::from_u64(and_shift as u64).unwrap();
            shift += 7;
            match b & 0x80 {
                0 => break,
                _ => continue
            }
        }

        bi.to_bigint().unwrap()
    }

    // Might drop this, we'll see
    fn next_int(&mut self) -> u64 {
        match self.next_varint().to_u64() {
            None => panic!("String length overflows integer bounds"),
            Some(x) => x
        }
    }

    fn next_str(&mut self) -> String {
        let size = self.next_int();
        match size {
            0   => String::from(""),
            _ => {
                // bs is bytes
                let bs: Vec<u8> = self.next_bytes(size as usize);
                // This will fail and die if the 'utf-8' is invalid, like Typhon
                String::from_utf8(bs).unwrap()
            }
        }
    }
}

pub fn parse_file(source_path: &str) {
    let mut mast_reader = match source_path {
        "" => {
            // Sadly, since stdin cannot be a File, dump stdin to a tmp file
            // then read it. It's lame...I hate this hack.
            // TODO: Fix this
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
