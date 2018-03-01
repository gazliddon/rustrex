// use mem::Memory;
use mem::{ MemoryIO };
use std::fmt;
use sha1::Sha1;

pub trait MemMapIO {
    fn add_memory(&mut self, mem : Box<MemoryIO> ) ;
}

pub struct MemMap {
    all_memory: Vec< Box<MemoryIO>>,
    name : String,
}

impl fmt::Debug for MemMap {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut strs : Vec<String> = Vec::new();

        for m in &self.all_memory {
            strs.push(m.get_name().clone())
        }

        write!(f, "{}", strs.join(" "))
    }
}

impl MemoryIO for MemMap {

    fn update_sha1(&self, digest : &mut Sha1) {
        for m in &self.all_memory {
            m.update_sha1(digest);
        }
    }

    fn upload(&mut self, addr : u16, data : &[u8]) {
        for (i, item) in data.iter().enumerate() {
            let a = addr.wrapping_add(i as u16);
            self.store_byte(a, *item)
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_range(&self) -> (u16, u16) {
        (0, 0xffff)
    }

    fn load_byte(&self, addr:u16) -> u8 {
        for m in &self.all_memory {
            if m.is_in_range(addr) {
                return m.load_byte(addr)
            }
        }
        0
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        for m in &mut self.all_memory {
            if m.is_in_range(addr) {
                m.store_byte(addr, val)
            }
        }
    }
}

impl MemMap {
    pub fn new() -> MemMap {
        MemMap {
            all_memory : Vec::new(),
            name : "all memory".to_string(),
        }
    }

    pub fn load_roms<'a>(&mut self, roms : &[(&'a str, u16)]) -> &mut Self{
        use utils::load_file;
        for rom in roms.iter() {
            let data = load_file(rom.0);
            self.upload(rom.1, &data);
        }
        self
    }
}

impl MemMapIO for MemMap {
    fn add_memory(&mut self, mem : Box<MemoryIO> ) {
        self.all_memory.push(mem)
    }
}

