#![allow(dead_code)]
#![allow(unused_variables)]
mod via;
mod memblock;
mod memmap;
mod addr;
mod symtab;
mod cpu;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::fs::File;
use std::io::Read;
use cpu::mem::MemoryIO;
use memblock::MemBlock;

fn load_file(file_name : &'static str) -> Vec<u8> {
    let mut file = File::open(file_name).unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}

static MEMS: &[(&'static str, bool, u16, u16)] = &[
   ("cart", false, 0, 0x8000 ),
   ("sysrom", false, 0xe000, 0x2000),
   ("ram", true, 0xc800, 0x800),
];

fn main() {
    use memmap::MemMap;
    use cpu::diss::Disassembler;
    use symtab::SymbolTable;

    let syms = SymbolTable::new("resources/syms.yaml");

    let to_load : &[(&'static str, u16)] = &[
        ("resources/rom.dat", 0xe000),
        ("resources/ROCKS.BIN", 0),
    ];

    let mut mm = MemMap::new();

    for &(name, rw, base, size) in MEMS {
        let mb = Box::new(MemBlock::new(name, rw, base, size));
        mm.add(mb);
    }

    for &(file_name, addr) in to_load.iter() {
        let data = load_file(file_name);
        mm.upload(addr, &data);
    }

    let mut diss = Disassembler::new(mm);

    diss.diss(0xf000,50);
}


