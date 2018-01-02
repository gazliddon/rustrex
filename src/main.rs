#![allow(dead_code)]
#![allow(unused_variables)]
mod via;
mod mem;
mod memmap;
mod diss;
mod addr;
mod symtab;
mod cpu2;
mod registers;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::fs::File;
use std::io::Read;
use mem::MemoryIO;

fn load_file(file_name : &'static str) -> Vec<u8> {

    let mut file = File::open(file_name).unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}

fn main() {

    use memmap::MemMap;
    use cpu2::diss::Disassembler;
    use symtab::SymbolTable;

    let syms = SymbolTable::new("resources/syms.yaml");

    let to_load : &[(&'static str, u16)] = &[
        ("resources/rom.dat", 0xe000),
        ("resources/ROCKS.BIN", 0),
    ];

    let mut mm = MemMap::new();

    for &(file_name, addr) in to_load.iter() {
        let data = load_file(file_name);
        mm.upload(addr, &data);
    }

    let mut diss = Disassembler::new(mm);

    diss.diss(0xf000,50);
}


