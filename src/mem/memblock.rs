use std::vec::Vec;
use mem::{ MemoryIO, MemMap, MemMapIO };
use sha1::Sha1;

pub struct MemBlock {
    pub read_only : bool,
    pub data : Vec<u8>,
    pub base : u16,
    pub size : usize,
    pub last_mem : u16,
    pub name : String,
}

impl MemBlock {

    pub fn new(name: &str, read_only : bool, base: u16, size: usize) -> MemBlock {

        let data = vec![0u8; size as usize];
        let last_mem = base.wrapping_add(size as u16).wrapping_sub(1);

        if last_mem < base {
            panic!("Trying to add memory > that 16 bit address space");
        }

        MemBlock {
            size: size,
            base: base,
            read_only: read_only,
            data: data,
            name: name.to_string(),
            last_mem: last_mem,
        }
    }

    pub fn from_data(addr : u16 ,name : &str, data : &[u8], writeable : bool ) -> MemBlock {
        let len = data.len() as u32;

        let last_byte = ( addr as u32 + len ) -1;

        if last_byte >= 0x1_0000 {
            println!("len: {:04x} base: {:04x} last: {:04x}", len ,addr, last_byte);
            assert!(last_byte < 0x1_0000);
        }

        let mut r = MemBlock::new(name, writeable, addr, data.len() );
        r.data = data.to_vec();
        r
    }
}

impl MemMap {
    pub fn add_mem_block(&mut self, name : &str, writable : bool, base : u16, size : usize) {
        let mb = Box::new(MemBlock::new(name, writable, base, size));
        self.add_memory(mb);
    }
}

impl MemoryIO for MemBlock {
    fn update_sha1(&self, digest : &mut Sha1) {
        digest.update(&self.data);
    }

    fn upload(&mut self, addr : u16, data : &[u8]) {
        panic!("not done")
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_range(&self) -> (u16, u16) {
        (self.base, self.last_mem)
    }

    fn load_byte(&mut self, addr:u16) -> u8 {
        assert!(addr >= self.base && addr <= self.last_mem);
        self.data[(addr - self.base) as usize]
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        assert!(addr >= self.base && addr <= self.last_mem);
        let idx = (addr - self.base) as usize;
        self.data[idx] = val;
    }
}





