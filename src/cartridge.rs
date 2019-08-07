use std::fs;
use std::io::prelude::*;

const CARTRIDGE_SIZE: usize = 0xE00;

pub struct Cartridge {
    pub contents: [u8; CARTRIDGE_SIZE],
    pub size: usize,
}

impl Cartridge {
    pub fn new(path: &str) -> Cartridge {
        let mut contents = [0; CARTRIDGE_SIZE];
        let mut cartridge_file = fs::File::open(path).expect("failed to open file");
        let size = cartridge_file.read(&mut contents).unwrap();
        Cartridge { 
            contents,
            size,
        }
    }
}