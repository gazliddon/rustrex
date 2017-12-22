use std::vec::Vec;

fn as_word(lo : u8, hi : u8) -> u16 {
    lo as u16 | (hi as u16) << 8
}

fn as_bytes(val : u16) -> (u8,u8) {
    ( (val&0xff) as u8, (val>>8) as u8 )
}

pub trait MemoryIO {
    fn get_name(&self) -> String {
        String::from("NO NAME")
    }

    fn get_range(&self) -> (u16, u16);

    fn is_in_range(&self, val : u16) -> bool {
        let (base, last) = self.get_range();
        (val >= base) && (val <= last)
    }

    fn load_byte(&self, addr:u16) -> u8;
        
    fn store_byte(&mut self, addr:u16, val:u8);

    fn store_word(&mut self, addr:u16, val:u16) {
        let (lo,hi) = as_bytes(val);
        self.store_byte(addr, lo);
        self.store_byte(addr+1, hi);
    }

    fn load_word(&self, addr:u16) -> u16 {
        as_word(self.load_byte(addr), self.load_byte(addr+1))
    }
}

pub struct Memory {
    pub read_only : bool,
    pub data : Vec<u8>,
    pub base : u16,
    pub size : u16,
    pub last_mem : u16,
    pub name : &'static str
}

impl Memory {
    pub fn new(name: &'static str, read_only : bool, base: u16, size: u16) -> Memory {

        let data = vec![0u8; size as usize];
        let last_mem = (base as u32) + (size as u32) - 1;

        Memory {
            size : size,
            base : base,
            read_only: read_only,
            data : data,
            name : name,
            last_mem : last_mem as u16,
        }
    }
}

impl MemoryIO for Memory {
    fn get_name(&self) -> String {
        String::from(self.name)
    }

    fn get_range(&self) -> (u16, u16) {
        (self.base, self.last_mem)
    }

    fn load_byte(&self, addr:u16) -> u8 {
        let idx = (addr - self.base) as usize;
        assert!(idx < self.size as usize);
        self.data[idx]
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        let idx = (addr - self.base) as usize;
        assert!(idx < self.size as usize);
        self.data[idx] = val;
    }
}





