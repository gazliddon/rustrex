use std::vec::Vec;

pub trait MemoryIO {

    fn load_byte(&self, addr:u16) -> u8;
    fn load_word(&self, addr:u16) -> u16;
    fn store_byte(&mut self, addr:u16, val:u8);
    fn store_word(&mut self, addr:u16, val:u16);
}

pub trait Size {
    fn is_in_range(&self, val : u16) -> bool;
}

pub struct Memory {
    pub read_only : bool,
    pub data : Vec<u8>,
    pub base : u16,
    pub size : u16,
    pub name : &'static str
}

impl Memory {

    pub fn new(name: &'static str, read_only : bool, base: u16, size: u16) -> Memory {

        let data = vec![0u8; size as usize];

        Memory {
            size : size,
            base : base,
            read_only: read_only,
            data : data,
            name : name,
        }
    }
}

impl Size for Memory {
    fn is_in_range(&self, val : u16) -> bool {
        (val >= self.base) && (val < (self.base + self.size))
    }
}

impl MemoryIO for Memory {
    fn load_byte(&self, addr:u16) -> u8 {
        let idx = (addr - self.base) as usize;
        assert!(idx < self.size as usize);
        self.data[idx]
    }

    fn load_word(&self, addr:u16) -> u16 {
        let lo = self.load_byte(addr ) as u32; 
        let hi = self.load_byte(addr+1) as u32;
        ((hi << 16) | lo) as u16

    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        let idx = (addr - self.base) as usize;
        assert!(idx < self.size as usize);
        self.data[idx] = val;
    }

    fn store_word(&mut self, addr:u16, val:u16) {
        self.store_byte(addr, (val&0xff) as u8);
        self.store_byte(addr, (val>>8) as u8);
    }
}





