// use cpu::Cpu;
use memmap::MemMap;
use mem::MemoryIO;

#[derive(Debug)]
pub enum Regs {
    D, A, B, X,Y,S,U, PC, DP
}

#[derive(Debug)]
pub enum IndexModes {
    ROff(Regs,i8),

    RPlus(Regs),     //               ,R+              2 0 |
    RPlusPlus(Regs), //               ,R++             3 0 |
    RSub(Regs),      //               ,-R              2 0 |
    RSubSub(Regs),   //               ,--R             3 0 |
    RZero(Regs),     //               ,R               0 0 |
    RAddB(Regs),     //             (+/- B),R          1 0 |
    RAddA(Regs),     //             (+/- A),R          1 0 |
    RAddi8(Regs),    //    (+/- 7 b  it offset),R      1 1 |
    RAddi16(Regs),   //      (+/- 15 bit offset),R     4 2 |
    RAddD(Regs),     //             (+/- D),R          4 0 |
    PCAddi8,         //      (+/- 7 bit offset),PC     1 1 |
    PCAddi16,        //      (+/- 15 bit offset),PC    5 2 |
    Illegal,         //              Illegal           u u |
    Ea,
}

pub fn get_format_string(mode : IndexModes ) -> String {
    panic!("dep")

}

bitflags! {
    pub struct IndexedFlags: u8 {
        const NOT_IMM     = 0b10000000;

        const R           = 0b01100000;
        const D           = 0b00111111;
        const OFFSET      = 0b00111111;
        const OFFSET_SIGN = 0b00100000;
        const IND         = 1 << 4;
        const TYPE        = 0b00001111;
        const IS_EA       = 0b10011111;
        const ALL         = 0b11111111;
    }
}

impl IndexedFlags {

    fn get_offset(&self) -> i8 {
        let mut v = self.bits & IndexedFlags::OFFSET.bits();

        v = if v & IndexedFlags::OFFSET_SIGN.bits() == IndexedFlags::OFFSET_SIGN.bits() {
            v | !IndexedFlags::OFFSET.bits()
        } else {
            v
        };

        v as i8
    }

    pub fn new(val : u8) -> Self {
        IndexedFlags {
            bits: val
        }
    }

    fn get_d(&self) -> u8{
        self.bits & IndexedFlags::D.bits()
    }

    pub fn is_ea(&self) -> bool {
        self.bits == IndexedFlags::IS_EA.bits()
    }

    pub fn is_indirect(&self) -> bool {
        (self.bits & IndexedFlags::IND.bits()) == IndexedFlags::IND.bits() 

    }

    fn not_imm(&self) -> bool {
        (self.bits & IndexedFlags::NOT_IMM.bits()) != 0
    }

    fn get_reg(&self) -> Regs {
        match ( self.bits & (IndexedFlags::R.bits()) ) >> 5{
            0 => Regs::X,
            1 => Regs::Y,
            2 => Regs::U,
            _ => Regs::S,
        }
    }

    fn get_type(&self) -> u8 {
        self.bits & IndexedFlags::TYPE.bits()
    }

    pub fn get_index_type(&self) -> IndexModes {

        let r = self.get_reg();

        if self.is_ea() {
            return IndexModes::Ea
        }

        if self.not_imm() {
            return match self.get_type() {
                0b0000 => IndexModes::RPlus(r),     //               ,R+              2 0 |
                0b0001 => IndexModes::RPlusPlus(r), //               ,R++             3 0 |
                0b0010 => IndexModes::RSub(r),      //               ,-R              2 0 |
                0b0011 => IndexModes::RSubSub(r),   //               ,--R             3 0 |
                0b0100 => IndexModes::RZero(r),     //               ,R               0 0 |
                0b0101 => IndexModes::RAddB(r),     //             (+/- B),R          1 0 |
                0b0110 => IndexModes::RAddA(r),     //             (+/- A),R          1 0 |
                0b0111 => IndexModes::Illegal,      //              Illegal           u u |
                0b1000 => IndexModes::RAddi8(r),    //    (+/- 7 b  it offset),R      1 1 |
                0b1001 => IndexModes::RAddi16(r),   //      (+/- 15 bit offset),R     4 2 |
                0b1010 => IndexModes::Illegal,      //              Illegal           u u |
                0b1011 => IndexModes::RAddD(r),     //             (+/- D),R          4 0 |
                0b1100 => IndexModes::PCAddi8,      //      (+/- 7 bit offset),PC     1 1 |
                0b1101 => IndexModes::PCAddi16,     //      (+/- 15 bit offset),PC    5 2 |
                0b1110 => IndexModes::Illegal,      //              Illegal           u u |
                _ => IndexModes::Illegal,

            }
        }

        IndexModes::ROff(r, self.get_offset())

    }
}

pub enum AddrModes {
    Illegal,
    Direct,
    Inherent,
    Variant,
    Relative,
    Indexed,
    Immediate,
    Extended,

    DecodedIndexed(IndexModes),
}

fn format_illegal(val : u16) -> String {
    String::from("???")
}

fn format_direct(val : u16) -> String {
    format!("${:02X}", val)
}

fn format_inherent(val : u16) -> String {
    String::from("TBD INHERENT")
}

fn format_variant(val : u16) -> String {
    String::from("TBD VARIANT" )
}

fn format_relative(val : u16) -> String {
    String::from("TBD RELATIVE")
}

fn format_indexed(val : u16) -> String {
    String::from("kjskjas")
}

fn format_immediate(val : u16) -> String {
    if 16 & 0xff00 != 0 {
        format!("#${:04X}", val)
    } else {
        format!("#${:02X}", val)
    }
}

fn format_extended(val : u16) -> String {
    format!("${:04X}", val)
}

pub fn format_operand( addr_mode : &AddrModes,  operand : u16) -> String {

    match *addr_mode {
        AddrModes::Illegal => format_illegal(operand),
        AddrModes::Direct => format_direct(operand),
        AddrModes::Inherent => format_inherent(operand),
        AddrModes::Variant => format_variant(operand),
        AddrModes::Relative => format_relative(operand),
        AddrModes::Indexed => format_indexed(operand),
        AddrModes::Immediate => format_immediate(operand),
        AddrModes::Extended => format_extended(operand),
        _ => String::from("error")
    }
}

pub fn fetch_illegal( mem : &MemMap, addr : u16) -> (u16, u16 ) {
    (0,0)
}

pub fn fetch_direct(mem : &MemMap, addr : u16) -> (u16, u16 ) {
    (1, mem.load_byte(addr) as u16 )
}

pub fn fetch_inherent(mem : &MemMap, addr : u16) -> (u16, u16 ) {
    (0, 0)
}

pub fn fetch_variant( mem : &MemMap, addr : u16) -> (u16, u16 ) {
    (0, 0)
}

pub fn fetch_relative(  mem : &MemMap, addr : u16) -> (u16, u16 ) {
    (1, 0)
}


pub fn fetch_immediate( mem : &MemMap, addr : u16) -> (u16, u16 ) {
    (0, mem.load_word(addr) )
}

pub fn fetch_extended( mem : &MemMap, addr : u16) -> (u16, u16 ) {
    (0, mem.load_word(addr) )
}

// pub fn fetch_operand( addr_mode : &AddrModes, mem : &MemMap, addr : u16) -> (u16, u16) {

//     match *addr_mode {
//         AddrModes::Illegal => fetch_illegal(mem, addr),
//         AddrModes::Direct => fetch_direct(mem, addr),
//         AddrModes::Inherent => fetch_inherent(mem, addr),
//         AddrModes::Variant => fetch_variant(mem, addr),
//         AddrModes::Relative => fetch_relative(mem, addr),
//         AddrModes::Indexed => fetch_indexed(mem, addr),
//         AddrModes::Immediate => fetch_immediate(mem, addr),
//         AddrModes::Extended => fetch_extended(mem, addr),
//         _ => (0,0),
//     }
// }





// 010D: 10 AE D8 09 LDY   [$09,U] 
// 1CCA: A0 B8 02    SUBA  [$02,Y]
// 10111000
// R = 01 = 1 = Y
// I = 1000 = 8 = ,R + 8 bit offset
// i = 1 = indirect
// [8bit,R] = [$2,Y]

#[test]
fn it_works() {

    let fl = IndexedFlags::new(0xb8);
    panic!("Flags!  {:?}", fl.get_index_type())

}


// pub trait AddrMode {
//     fn name(&self) -> &'static str;
//     fn fetch (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> u16;
//     fn store (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16, val : u16);
//     fn format_str(&self, val : u16) -> String;
// }

// pub struct Illegal {}
// pub struct Direct {}
// pub struct Inherent {}
// pub struct Variant {}
// pub struct Relative {}
// pub struct Indexed {}
// pub struct Immediate {}
// pub struct Extended {}

// impl AddrMode for Illegal {
//     fn name(&self) -> &'static str { "ILLEGAL" }

//     fn fetch (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> u16 {
//         0
//     }

//     fn store (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16, val : u16) {
//     }

//     fn format_str(&self, val : u16) -> String {
//         String::from("???")
//     }
// }

// impl AddrMode for Direct {
//     fn name(&self) -> &'static str { "DIRECT" }

//     fn fetch (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> u16 {
//         0
//     }

//     fn store (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16, val : u16) {
//     }

//     fn format_str(&self, val : u16) -> String {
//         format!("${:02X}", val)
//     }
// }

// impl AddrMode for Inherent {
//     fn name(&self) -> &'static str { "INHERENT" }

//     fn fetch (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> u16 {
//         0
//     }

//     fn store (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16, val : u16) {
//     }

//     fn format_str(&self, val : u16) -> String {
//         String::new()
//     }
// }

// impl AddrMode for Variant {
//     fn name(&self) -> &'static str { "VARIANT" }

//     fn fetch (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> u16 {
//         0
//     }

//     fn store (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16, val : u16) {
//     }

//     fn format_str(&self, val : u16) -> String {
//         String::new()
//     }
// }

// impl AddrMode for Relative {
//     fn name(&self) -> &'static str { "RELATIVE" }

//     fn fetch (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> u16 {
//         0
//     }

//     fn store (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16, val : u16) {
//     }

//     fn format_str(&self, val : u16) -> String {
//         String::new()
//     }
// }

// impl AddrMode for Indexed {
//     fn name(&self) -> &'static str { "INDEXED" }

//     fn fetch (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> u16 {
//         0
//     }

//     fn store (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16, val : u16) {
//     }

//     fn format_str(&self, val : u16) -> String {
//         String::new()
//     }
// }

// impl AddrMode for Immediate {

//     fn name(&self) -> &'static str { "IMMEDIATE" }

//     fn fetch (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> u16 {
//         mem.load_word(addr)
//     }

//     fn store (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16, val : u16) {
//     }

//     fn format_str(&self, val : u16) -> String {
//         format!("#${:02X}", val)
//     }
// }

// impl AddrMode for Extended {

//     fn name(&self) -> &'static str { "EXTENDED" }

//     fn fetch (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> u16 {
//         mem.load_word(addr)
//     }

//     fn store (&self, cpu : &mut Cpu, mem : &mut MemMap, addr : u16, val : u16) {
//     }

//     fn format_str(&self, val : u16) -> String {
//         format!("#${:02X}", val)
//     }
// }

