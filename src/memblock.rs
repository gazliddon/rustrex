use std::vec::Vec;
use cpu::mem::MemoryIO;

pub struct MemBlock {
    pub read_only : bool,
    pub data : Vec<u8>,
    pub base : u16,
    pub size : u16,
    pub last_mem : u16,
    pub name : &'static str
}

impl MemBlock {
    pub fn new(name: &'static str, read_only : bool, base: u16, size: u16) -> MemBlock {

        let data = vec![0u8; size as usize];
        let last_mem = base.wrapping_add(size).wrapping_sub(1);

        if last_mem < base {
            panic!("Trying to add memory > that 16 bit address space");
        }

        MemBlock {
            size: size,
            base: base,
            read_only: read_only,
            data: data,
            name: name,
            last_mem: last_mem,
        }
    }
}

impl MemoryIO for MemBlock {

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





