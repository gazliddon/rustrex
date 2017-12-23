use memmap::MemMap;
use mem::MemoryIO;

bitflags! {
    pub struct Flags: u8 {
        const ZERO      = 0b00000000;
        const E         = 0b00000001;
        const F         = 0b00000010;
        const H         = 0b00000100;
        const I         = 0b00001000;
        const N         = 0b00010000;
        const Z         = 0b00100000;
        const V         = 0b01000000;
        const C         = 0b10000000;
    }
}

#[derive(Debug)]
pub struct Regs {
    pub d : u16,
    pub a : u8,
    pub b : u8,
    pub x : u16,
    pub y : u16,
    pub u : u16,
    pub s : u16,
    pub pc: u16,
    pub dp: u8,
    pub flags: Flags,
}

impl Regs {
    pub fn new() -> Regs {
        Regs {
            d : 0,
            a : 0,
            b : 0,
            x : 0,
            y : 0,
            u : 0,
            s : 0,
            pc: 0,
            dp: 0,
            flags: Flags::ZERO
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    pub regs : Regs,
}


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs : Regs::new()
        }

    }

    // pub fn get_flag(&self, flag : FlagBits) -> bool {
    //     ( self.regs.flags & flag ) == 0
    // }

    // pub fn set_flag(&mut self, flag : FlagBits) {
    //     self.regs.flags |= flag
    // }

    pub fn next(&mut self, mem : &mut MemMap) -> bool {

        let ins = mem.load_byte(self.regs.pc);

        if ins == 0x10 {

        } else {

            match ins {

                0x00 => println!("neg"),
                0x03 => println!("com"),
                0x06 => println!("lsr"),
                0x07 => println!("ror"),
                0x08 => println!("asl/lsl"),
                0x0a => println!("rol"),
                0x0c => println!("dec"),
                0x0d => println!("inc"),
                0x0e => println!("tst"),
                0x0f => println!("jmp"),

                _ => println!("illegal instructions {}", ins)
            }

        }

        true
    }

}

