#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

mod via;
mod memblock;
mod memmap;
mod addr;
mod symtab;
mod cpu;
mod utils;

use symtab::SymbolTable;
use memmap::MemMap;
use cpu::diss::Disassembler;

static MEMS: &[(&'static str, bool, u16, u16)] = &[
   ("cart"  , false, 0     , 0x8000 ),
   ("sysrom", false, 0xe000, 0x2000) ,
   ("ram"   , true , 0xc800, 0x800)  ,
];

static ROMS : &[(&'static str, u16)] = &[
    ( "resources/rom.dat", 0xe000 ),
    ( "resources/ROCKS.BIN", 0 ),
];

fn main() {
    let syms = SymbolTable::new("resources/syms.yaml");

    let mut mm = MemMap::new();

    for &(name, rw, base, size) in MEMS {
        mm.add_mem_block(name, rw, base, size)
    }

    mm.load_roms(ROMS);

    let mut diss = Disassembler::new(mm);

    diss.diss(0xf000,50, Some(&syms));
}

