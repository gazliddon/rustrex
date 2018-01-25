use mem::MemoryIO;
use cpu::{ Regs, InstructionDecoder };

trait AddressLines {
    fn get(&self) -> u16;

    fn load_byte<M: MemoryIO>(&self, mem : &M, regs : &mut Regs) -> u8 {
        mem.load_byte(self.get())
    }

    fn load_word<M: MemoryIO>(&self, mem : &M, regs : &mut Regs) -> u16 {
        mem.load_word(self.get())
    }
}


struct Direct {
    addr : u16,
}

impl Direct {
    fn new<M: MemoryIO>(mem : &M, regs : &Regs, ins : &mut InstructionDecoder) -> Direct {
        let index = ins.fetch_byte(mem) as u16;
        Direct { 
            addr: regs.get_dp_ptr().wrapping_add(index)
        }
    }
}

impl AddressLines for Direct {
    fn get(&self) -> u16 {
        self.addr
    }
}


struct Extended {}

struct Immediate {}

struct Inherent {}

struct Indexed {}

struct Relative {}



