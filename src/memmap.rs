// use mem::Memory;
use mem::MemoryIO;
use mem::Memory;

// Genericise TODO

fn as_word(lo : u8, hi : u8) -> u16 {
    lo as u16 | (hi as u16) << 8
}

fn as_bytes(val : u16) -> (u8,u8) {
    ( (val&0xff) as u8, (val>>8) as u8 )
}

pub struct MemMap {
    all_memory: Vec< Box<MemoryIO> >
}

impl MemoryIO for MemMap {
    fn is_in_range(&self, val : u16) -> bool {
        true
    }

    fn load_byte(&self, addr:u16) -> u8 {
        for m in self.all_memory.iter() {
            if m.is_in_range(addr) {
                return m.load_byte(addr)
            }
        }
        0
    }

    fn load_word(&self, addr:u16) -> u16 {
        as_word(self.load_byte(addr), self.load_byte(addr+1))
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        for m in self.all_memory.iter_mut() {
            if m.is_in_range(addr) {
                m.store_byte(addr, val)
            }
        }
    }

    fn store_word(&mut self, addr:u16, val:u16) {
        let (lo,hi) = as_bytes(val);
        self.store_byte(addr, lo);
        self.store_byte(addr+1, hi);
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
}

