#![allow(dead_code)]
#![allow(unused_variables)]

mod via;
mod mem;
mod memmap;
mod cpu;
mod isa;
mod machine;

#[macro_use]
extern crate bitflags;

use std::fs::File;
use std::io::Read;

fn main() {

    let file_name = "resources/ROCKS.BIN";

    let mut file = File::open(file_name).unwrap();
 
    let mut contents: Vec<u8> = Vec::new();
    // Returns amount of bytes read and append the result to the buffer
    let result = file.read_to_end(&mut contents).unwrap();

    println!("Read {} bytes", result);
}


