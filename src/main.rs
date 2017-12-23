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
use mem::MemoryIO;


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
        println!("uploaded {} to {:04X} ", file_name, addr)
    }
}

fn hex_dump(mac : &Machine, addr : u16) {
}


impl isa::Ins {
    fn diss(&self, addr : u16) -> String {

        String::from("nonon")
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


    let mut addr  = 0xf000u16;


    for i in 0..16 {
        let ins = m.fetch_instruction(addr);

        let b = m.mem.load_byte(addr);

        println!("0x{:04x} 0x{:02x} {}   {:02x}", addr, b, ins.op.mnenomic, ins.op_code);

        addr = ( addr as u32 + ins.bytes as u32 ) as u16;
    }




    // let mut addr : u16 = 0xf000; 

    // for i in 0..9 {
    //     let ins = m.fetch_instruction(addr);

    //     println!("${:04X } {:?}", addr, ins);

    //     addr = addr + ins.bytes as u16;
    // }

}


