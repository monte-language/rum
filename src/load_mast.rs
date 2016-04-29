use std::io::BufReader;
use std::io::prelude::*;

use bincode::rustc_serialize::decode;
use num::Zero;
use num::bigint::{BigInt, BigUint, ToBigInt};
use num::traits::{FromPrimitive, ToPrimitive};


const MAGIC: &'static [u8; 9] = b"Mont\xe0MAST";
const MAGIC_V: &'static [u8; 1] = b"\x00";


pub trait MastReader {
    fn check_magic_numbers(&mut self) -> bool;
    fn take_n_bytes(&mut self, n: usize) -> Vec<u8>;
    fn next_byte(&mut self) -> u8;
    fn next_bytes(&mut self, count: usize) -> Vec<u8>;
    fn next_double(&mut self) -> f64;
    fn next_varint(&mut self) -> BigInt;
    fn next_int(&mut self) -> u64;
    fn next_str(&mut self) -> String;
}

impl <R: Read> MastReader for BufReader<R> {
    fn check_magic_numbers(&mut self) -> bool {
        let mut nums = [0u8; 10];
        self.take(10).read(&mut nums).unwrap();
        for i in 0..9 {
            if nums[i] != MAGIC[i] {
                panic!("Filetype is not Monte AST")
            }
        }
        if nums[9] != MAGIC_V[0] {
            panic!("Wrong Monte AST version")
        }
        true
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
        let d: Option<f64> = decode(&bytes).unwrap();
        match d {
            Some(dbl) => dbl,
            None => panic!("The mast lies!")
        }
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

pub trait MastContext {
    fn new(n: bool) -> Context;
    fn decode_next_tag(&mut self, stream: &mut MastReader);
}

struct Context {
    // Print fancy things about the MAST?
    noisy: bool,
}

impl MastContext for Context {
    fn new(n: bool) -> Context {
        Context {
            noisy: n,
        }
    }

    fn decode_next_tag(&mut self, stream: &mut MastReader) {
        let tag = stream.next_byte();
        match tag {
            b'L' => {
                let literal_tag = stream.next_byte();
                match literal_tag {
                    b'C' => panic!("Literal '{:?}' not yet implemented!", literal_tag),
                    b'D' => panic!("Literal '{:?}' not yet implemented!", literal_tag),
                    b'I' => panic!("Literal '{:?}' not yet implemented!", literal_tag),
                    b'N' => panic!("Literal '{:?}' not yet implemented!", literal_tag),
                    b'S' => panic!("Literal '{:?}' not yet implemented!", literal_tag),
                    _ => panic!("'{:?}' Unknown literal tag!", literal_tag),
                }
            },
            b'P' => {
                let pattern_tag = stream.next_byte();
                match pattern_tag {
                    b'F' => panic!("Pattern '{:?}' not yet implemented!", pattern_tag),
                    b'I' => panic!("Pattern '{:?}' not yet implemented!", pattern_tag),
                    b'V' => panic!("Pattern '{:?}' not yet implemented!", pattern_tag),
                    b'L' => panic!("Pattern '{:?}' not yet implemented!", pattern_tag),
                    b'A' => panic!("Pattern '{:?}' not yet implemented!", pattern_tag),
                    b'B' => panic!("Pattern '{:?}' not yet implemented!", pattern_tag),
                    _ => panic!("'{:?}' Unknown pattern tag!", pattern_tag),
                }
            },
            b'N' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'B' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'S' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'C' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'D' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'e' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'E' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'O' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'M' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'R' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'A' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'F' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'Y' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'H' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'I' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'T' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            b'X' => {
                panic!("{:?}' Not yet implemented!", tag);
            },
            _ => {
                panic!("{:?}' Unknown Tag!", tag);
            },
        }
    }
}
