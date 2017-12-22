// use mem::Memory;
use mem::MemoryIO;
use mem::Memory;

// Genericise TODO


pub struct MemMap {
    all_memory: Vec< Box<MemoryIO> >
}

impl MemoryIO for MemMap {
    fn get_name(&self) -> String {
        String::from("Entire Address Range")
    }

    fn get_range(&self) -> (u16, u16) {
        (0, 0xffff)
    }

    fn load_byte(&self, addr:u16) -> u8 {
        for m in self.all_memory.iter() {
            if m.is_in_range(addr) {
                return m.load_byte(addr)
            }
        }
        0
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        for m in self.all_memory.iter_mut() {
            if m.is_in_range(addr) {
                m.store_byte(addr, val)
            }
        }
    }
}

static MEMS: &[(&'static str, bool, u16, u16)] = &[
    ("cart", false, 0, 0x8000 ),
    ("sysrom", false, 0xe000, 0x2000),
];

impl MemMap {
    pub fn new() -> MemMap {

        let mut v : Vec<Box<MemoryIO>> = Vec::new();

        for &(name, writeable, base, size) in MEMS {

            let m1 = Memory::new(name, writeable, base, size);

            v.push(Box::new(m1));
        }

        MemMap {
            all_memory : v
        }
    }

    pub fn dump(&self) {
        for i in &self.all_memory {
        }
 
    }
}

