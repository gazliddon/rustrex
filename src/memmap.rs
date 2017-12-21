use mem::Size;
use mem::Memory;
use mem::MemoryIO;

use via;

pub struct MemMap {
    rom: Memory,
    ram: Memory,
    via : via::Via,
}

impl MemMap {
    pub fn get_mem_mut(&mut self,addr:u16) -> &mut Memory {
        if self.rom.is_in_range(addr) {
            &mut self.rom
        } else if self.rom.is_in_range(addr) {
            &mut self.ram
        } else {
            &mut self.ram
        }
    } 

    pub fn get_mem(&self,addr:u16) -> &Memory {
        if self.rom.is_in_range(addr) {
            &self.rom
        } else if self.rom.is_in_range(addr) {
            &self.ram
        } else {
            &self.ram
        }
    }
}

impl MemoryIO for MemMap {

    fn load_byte(&self, addr:u16) -> u8 {
        let mem = self.get_mem(addr);
        mem.load_byte(addr)
    }

    fn load_word(&self, addr:u16) -> u16 {
        let mem = self.get_mem(addr);
        mem.load_word(addr)
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        let mem = self.get_mem_mut(addr);
        mem.store_byte(addr,val);
    }

    fn store_word(&mut self, addr:u16, val:u16) {
        let mem = self.get_mem_mut(addr);
        mem.store_word(addr,val);
    }
}

impl MemMap {
    pub fn new() -> MemMap {
        MemMap {
            rom : Memory::new("rom", true, 0xc000, 8*1024),
            ram : Memory::new("ram", false, 0xc000, 8*1024),
            via : via::Via {},
        }
    }

    fn fetch_at_pc(&self) -> u8 {
        0
    }

}

