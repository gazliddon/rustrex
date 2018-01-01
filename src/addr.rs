// use cpu::Cpu;
use memmap::MemMap;
use mem::MemoryIO;

#[derive(Debug)]
pub enum Regs {
    D, A, B, X,Y,S,U, PC, DP
}

#[derive(Debug)]
pub enum IndexModes {
    ROff { reg : Regs, off : i8  },
    Rpp(Regs, bool),
    Rd(Regs, bool),
    Rdd(Regs, bool),
    Rzero(Regs, bool),
    RplusB(Regs, bool),
    Rplus8(Regs, bool),
    Rplus16(Regs, bool),
    RplusA(Regs, bool),
    RplusD(Regs, bool),
    PCplus8(bool),
    PCplus16(bool),
    Ea(bool),
    Dunno,
}

pub fn get_format_string(mode : IndexModes ) -> String {

    match mode {
        IndexModes::Rpp(r, i) => format!(",{:?}++",r),
        IndexModes::Rd(r, i) => format!(",-{:?}",r),
        IndexModes::Rdd(r, i) => format!(",--{:?}",r),
        IndexModes::Rzero(r, i) => format!(",{:?}",r),
        IndexModes::RplusB(r,i) => format!("B,{:?}", r),
        IndexModes::Rplus8(r,i) => format!("{},{:?}","{}", r),
        IndexModes::Rplus16(r,i) => format!("{},{:?}","{}", r),
        IndexModes::RplusA(r,i) => format!("A,{:?}", r),
        IndexModes::RplusD(r,i) => format!("D,{:?}", r),
        IndexModes::PCplus8(i) => format!("0x{},PC", "{:02X}"),
        IndexModes::PCplus16(i) =>format!("0x{},PC", "{:04X}"),
        _ => String::from("dunno")
    }
}

bitflags! {
    pub struct IndexedFlags: u8 {
        const IMM  = 0b10000000;
        const R    = 0b01100000;
        const D    = 0b00111111;
        const IND  = 0b00100000;
        const TYPE = 0b00001111;
        const ALL = 0b11111111;
    }
}

impl IndexedFlags {
    pub fn get_format_string(&self) -> String {
        String::from("oh dear!")
    }

    pub fn new(val : u8) -> Self {
        IndexedFlags {
            bits: val
        }
    }

    pub fn get_d(&self) -> u8{
        self.bits & IndexedFlags::D.bits()
    }

    pub fn is_indirect(&self) -> bool {
        (self.bits & IndexedFlags::IND.bits()) == IndexedFlags::IND.bits()
    }

    pub fn has_imm(&self) -> bool {
        (self.bits & IndexedFlags::IMM.bits()) == IndexedFlags::IMM.bits()
    }

    pub fn get_reg(&self) -> Regs {
        match self.bits & (IndexedFlags::R.bits()) {
            0b0000000 => Regs::X,
            0b0100000 => Regs::Y,
            0b1000000 => Regs::S,
            _ => Regs::U,
        }
    }

    pub fn get_index_type(&self) -> IndexModes {
        let r = self.get_reg();
        let v = self.bits & IndexedFlags::TYPE.bits();
        let i = self.is_indirect();

        match v {
            0b0000 => IndexModes::Rpp(r,i),
            0b0001 => IndexModes::Rd(r,i),
            0b0011 => IndexModes::Rdd(r,i),
            0b0100 => IndexModes::Rzero(r,i),
            0b0101 => IndexModes::RplusB(r,i),
            0b0110 => IndexModes::RplusA(r,i),
            0b1000 => IndexModes::Rplus8(r,i),
            0b1001 => IndexModes::Rplus16(r,i),
            0b1011 => IndexModes::RplusD(r,i),
            0b1100 => IndexModes::PCplus8(i),
            0b1101 => IndexModes::PCplus16(i),
            0b1111 => IndexModes::Ea(i),
            _ => IndexModes::Dunno,
        }
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

pub fn fetch_indexed(  mem : & MemMap, addr : u16) -> (u16, u16 ) {
    let idx_flags = IndexedFlags::new(mem.load_byte(addr));

    match idx_flags.get_index_type() {
        IndexModes::PCplus8(_) => (2,mem.load_byte(addr+1) as u16),
        IndexModes::PCplus16(_) => (3, mem.load_word(addr+1)),
        IndexModes::Ea(_) => (3, mem.load_word(addr+1)),
        _ => (1,0)
    }
}

pub fn fetch_immediate( mem : &MemMap, addr : u16) -> (u16, u16 ) {
    (0, mem.load_word(addr) )
}

pub fn fetch_extended( mem : &MemMap, addr : u16) -> (u16, u16 ) {
    (0, mem.load_word(addr) )
}

pub fn fetch_operand( addr_mode : &AddrModes, mem : &MemMap, addr : u16) -> (u16, u16) {

    match *addr_mode {
        AddrModes::Illegal => fetch_illegal(mem, addr),
        AddrModes::Direct => fetch_direct(mem, addr),
        AddrModes::Inherent => fetch_inherent(mem, addr),
        AddrModes::Variant => fetch_variant(mem, addr),
        AddrModes::Relative => fetch_relative(mem, addr),
        AddrModes::Indexed => fetch_indexed(mem, addr),
        AddrModes::Immediate => fetch_immediate(mem, addr),
        AddrModes::Extended => fetch_extended(mem, addr),
        _ => (0,0),
    }
}





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

