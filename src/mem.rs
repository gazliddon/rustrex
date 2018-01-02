use std::vec::Vec;
use std;


pub fn to_mem_range( address : u16, size :u16 ) -> std::ops::Range<u32> {
    use std::cmp::min;

    let last_mem = address as u32 + size as u32;

    (address as u32 .. min(0x10000, last_mem) )
}

pub fn as_word(lo : u8, hi : u8) -> u16 {
    lo as u16 | (hi as u16) << 8
}

pub fn as_bytes(val : u16) -> (u8,u8) {
    ( (val&0xff) as u8, (val>>8) as u8 )
}

pub trait MemoryIO {

    fn upload(&mut self, addr : u16, data : &[u8]);

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
        self.store_byte(addr, hi);
        self.store_byte(addr.wrapping_add(1), lo);
    }

    fn load_word(&self, addr:u16) -> u16 {
        let lo = self.load_byte(addr.wrapping_add(1));
        let hi = self.load_byte(addr);
        as_word(lo, hi)
    }

    fn get_mem_as_str(&self, addr:u16, size:u16 ) -> String {
        let a32 = addr as u32;

        let r = to_mem_range( addr, size);

        let mut v : Vec<String> = Vec::new();

        for a in r {
            let b = self.load_byte(a as u16);
            let t = format!("{:02X}", b);
            v.push(t);
        }

        v.join(" ")
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
        let last_mem = base.wrapping_add(size).wrapping_sub(1);

        if last_mem < base {
            panic!("Trying to add memory > that 16 bit address space");
        }

        Memory {
            size: size,
            base: base,
            read_only: read_only,
            data: data,
            name: name,
            last_mem: last_mem,
        }
    }
}

impl MemoryIO for Memory {

    fn upload(&mut self, addr : u16, data : &[u8]) {
        panic!("not done")
    }

    fn get_name(&self) -> String {
        String::from(self.name)
    }

    fn get_range(&self) -> (u16, u16) {
        (self.base, self.last_mem)
    }

    fn load_byte(&self, addr:u16) -> u8 {
        assert!(addr >= self.base && addr <= self.last_mem);
        self.data[(addr - self.base) as usize]
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        assert!(addr >= self.base && addr <= self.last_mem);
        let idx = (addr - self.base) as usize;
        self.data[idx] = val;
    }
}





