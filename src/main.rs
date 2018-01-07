#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

mod cpu;
mod mem;

mod via;
mod memblock;
mod memmap;
mod symtab;
mod utils;

use symtab::SymbolTable;
use memmap::MemMap;
use cpu::Disassembler;

static MEMS: &[(&'static str, bool, u16, u16)] = &[
   ("cart"  , false, 0     , 0x8000 ),
   ("sysrom", false, 0xe000, 0x2000) ,
   ("ram"   , true , 0xc800, 0x800)  ,
];

static ROMS : &[(&'static str, u16)] = &[
    ( "resources/rom.dat", 0xe000 ),
    ( "utils/6809/6809all.raw", 0x1000 ),
];

fn main() {
    let syms = SymbolTable::new("resources/syms.yaml");

    let mut mm = MemMap::new();

    for &(name, rw, base, size) in MEMS {
        mm.add_mem_block(name, rw, base, size)
    }

    mm.load_roms(ROMS);

    let mut diss = Disassembler::new(mm);

    // diss.diss(0x104d,30, Some(&syms));
    diss.diss(0xf000,30, Some(&syms));
}

