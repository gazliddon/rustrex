use mem::MemoryIO;
use cpu::{ Regs, InstructionDecoder };

trait AddressLines {
    fn get(&self) -> u16;

    fn load_byte<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u8 {
        mem.load_byte(self.get())
    }

    fn load_word<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u16 {
        mem.load_word(self.get())
    }
}

////////////////////////////////////////////////////////////////////////////////
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


////////////////////////////////////////////////////////////////////////////////
struct Extended {

}
impl Extended {
    fn new<M: MemoryIO>(mem : &M, regs : &Regs, ins : &mut InstructionDecoder) -> Extended {
        Extended {
        }
    }
}

impl AddressLines for Extended {
    fn get(&self) -> u16 {
        panic!("no")
    }

    fn load_byte<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u8 {
        panic!("no")
    }

    fn load_word<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u16 {
        panic!("no")
    }

}

////////////////////////////////////////////////////////////////////////////////
struct Immediate {

}
impl Immediate {
    fn new<M: MemoryIO>(mem : &M, regs : &Regs, ins : &mut InstructionDecoder) -> Immediate {
        Immediate {
        }
    }
}

impl AddressLines for Immediate {
    fn get(&self) -> u16 {
        panic!("no")
    }

    fn load_byte<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u8 {
        panic!("no")
    }

    fn load_word<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u16 {
        panic!("no")
    }

}

////////////////////////////////////////////////////////////////////////////////
struct Inherent {

}
impl Inherent {
    fn new<M: MemoryIO>(mem : &M, regs : &Regs, ins : &mut InstructionDecoder) -> Inherent {
        Inherent {
        }
    }

}
impl AddressLines for Inherent {
    fn get(&self) -> u16 {
        panic!("no")
    }

    fn load_byte<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u8 {
        panic!("no")
    }

    fn load_word<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u16 {
        panic!("no")
    }

}

////////////////////////////////////////////////////////////////////////////////
struct Indexed {
}

impl Indexed {
    fn new<M: MemoryIO>(mem : &M, regs : &Regs, ins : &mut InstructionDecoder) -> Indexed {
        Indexed {
        }

    }
}

impl AddressLines for Indexed {
    fn get(&self) -> u16 {
        panic!("no")
    }

    fn load_byte<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u8 {
        panic!("no")
    }

    fn load_word<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u16 {
        panic!("no")
    }

}

////////////////////////////////////////////////////////////////////////////////
struct Relative {

}
impl Relative {
    fn new<M: MemoryIO>(mem : &M, regs : &Regs, ins : &mut InstructionDecoder) -> Relative {
        Relative {
        }

    }
}
impl AddressLines for Relative {
    fn get(&self) -> u16 {
        panic!("no")
    }

    fn load_byte<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u8 {
        panic!("no")
    }

    fn load_word<M: MemoryIO>(&self, mem : &M, regs : &mut Regs, ins : &InstructionDecoder) -> u16 {
        panic!("no")
    }

}

////////////////////////////////////////////////////////////////////////////////


