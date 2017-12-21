#![allow(dead_code)]
#![allow(unused_variables)]

mod via;
mod mem;
mod memmap;
mod cpu;
mod isa;
mod machine;
mod diss;

#[macro_use]
extern crate bitflags;

use std::fs::File;
use std::io::Read;
use machine::Machine;

fn load_file(file_name : &'static str) -> Vec<u8> {
    let mut file = File::open(file_name).unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}

impl Machine {
    fn upload_file(&mut self, file_name : &'static str,  addr : u16) {
        let data = load_file(file_name);
        self.upload(&data, addr);
    }
}

fn main() {
    let to_load : &[(&'static str, u16)] = &[
        ("resources/rom.dat", 0xe000),
        ("resources/ROCKS.BIN", 0),
    ];

    let mut m = Machine::new();

    for &(file_name, addr) in to_load.iter() {
        m.upload_file(file_name, addr)
    }

    for a in 0..5 {

    }
}


