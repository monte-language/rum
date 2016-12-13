use std::io::BufReader;
use std::io::prelude::*;

use bincode::rustc_serialize::decode;
use num::Zero;
use num::bigint::{BigInt, BigUint, ToBigInt};
use num::traits::{FromPrimitive, ToPrimitive};

use nodes::*;

const MAGIC: &'static [u8; 9] = b"Mont\xe0MAST";
const MAGIC_V: &'static [u8; 1] = b"\x00";


pub trait MastReader {
    fn check_magic_numbers(&mut self) -> bool;
    fn take_n_bytes(&mut self, n: usize) -> Vec<u8>;
    fn next_byte(&mut self) -> u8;
    fn next_bytes(&mut self, count: usize) -> Vec<u8>;
    fn next_double(&mut self) -> f64;
    fn next_varint(&mut self) -> Option<BigInt>;
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
        println!("n: {:?}", n);
        let mut bytes = Vec::new();
        bytes.resize(n, 0);
        let _ = self.read_exact(bytes.as_mut_slice());
        let hex_out: String = bytes.iter().map(|b| format!("{}", *b as char)).collect();
        println!("bytes: {}", hex_out);
        bytes
    }

    fn next_byte(&mut self) -> u8 {
        match self.take_n_bytes(1).pop() {
            Some(x) => x,
            None => panic!("Well shit, nothin there"),
        }
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

    fn next_varint(&mut self) -> Option<BigInt> {
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

        bi.to_bigint()
    }

    // Might drop this, we'll see
    fn next_int(&mut self) -> u64 {
        match self.next_varint() {
            Some(bi) => match bi.to_u64() {
                Some(x) => x,
                None => panic!("String length overflows integer bounds")
            },
            None => panic!("Unable to parse integer.")
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
    fn new(n: bool, stream: Box<BufReader<Box<Read>>>) -> Context;
    fn decode_next_tag(&mut self);
}

pub struct Context {
    // Print fancy things about the MAST?
    noisy: bool,
    stream: Box<MastReader>,
    expers: Box<Vec<Box<Expr>>>,
    patts: Box<Vec<Box<FinalPattern>>>,
}

impl MastContext for Context {
    fn new(n: bool, stream: Box<BufReader<Box<Read>>>) -> Context {
        Context {
            noisy: n,
            stream: stream as Box<MastReader>,
            expers: Box::new(Vec::new()),
            patts: Box::new(Vec::new()),
        }
    }

    fn decode_next_tag(&mut self) {
        let tag: u8 = self.stream.next_byte();
        match tag {
            b'L' => {
                let literal_tag = self.stream.next_byte();
                match literal_tag {
                    b'C' => {
                        let mut buf = vec![self.stream.next_byte()];
                        let mut utf8 = String::from_utf8(buf.clone());
                        let c: char;
                        if utf8.is_err() {
                            while utf8.unwrap_err().utf8_error().valid_up_to() == 0 {
                                buf.push(self.stream.next_byte());
                                utf8 = String::from_utf8(buf.clone());
                            }
                            c = String::from_utf8(buf.clone()).unwrap().pop().unwrap();
                        } else {
                            c = utf8.unwrap().pop().unwrap();
                        }
                        self.expers.push(Box::new(CharExpr::new(c.clone())));
                    },
                    b'D' => panic!("Literal '{}' not yet implemented!", literal_tag as char),
                    b'I' => panic!("Literal '{}' not yet implemented!", literal_tag as char),
                    b'N' => {
                        let n = NullExpr;
                        n.auditor_stamps();
                        self.expers.push(Box::new(n));
                    }
                    b'S' => panic!("Literal '{}' not yet implemented!", literal_tag as char),
                    _ => panic!("'{}' Unknown literal tag!", literal_tag as char),
                }
            },
            b'P' => {
                let pattern_tag = self.stream.next_byte();
                match pattern_tag {
                    b'F' => panic!("Pattern '{}' not yet implemented!", pattern_tag as char),
                    b'I' => panic!("Pattern '{}' not yet implemented!", pattern_tag as char),
                    b'V' => panic!("Pattern '{}' not yet implemented!", pattern_tag as char),
                    b'L' => panic!("Pattern '{}' not yet implemented!", pattern_tag as char),
                    b'A' => panic!("Pattern '{}' not yet implemented!", pattern_tag as char),
                    b'B' => panic!("Pattern '{}' not yet implemented!", pattern_tag as char),
                    _ => panic!("'{}' Unknown pattern tag!", pattern_tag as char),
                }
            },
            b'N' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'B' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'S' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'C' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'D' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'e' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'E' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'O' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'M' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'R' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'A' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'F' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'Y' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'H' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'I' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'T' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            b'X' => {
                panic!("{:?} Not yet implemented!", tag as char);
            },
            _ => {
                panic!("{:?} Unknown Tag!", tag as char);
            },
        }
    }
}

pub fn load(file: Box<BufReader<Box<Read>>>) {
    let mut ctx = Context::new(true, file);
    match ctx.stream.check_magic_numbers() {
        true => {
            loop {
                ctx.decode_next_tag();
            }
        },
        _ => {
            panic!("Woh, we broke through a panic...");
        },
    }
}
