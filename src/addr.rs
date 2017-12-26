use cpu::Cpu;
use memmap::MemMap;
use mem::MemoryIO;

pub enum Regs {
    D, A, B, X,Y,S,U, PC, DP
}

bitflags! {
    pub struct IndexedFlags: u8 {
        const ZERO = 0;
        const IMM  = 0b10000000;
        const R    = 0b01100000;
        const D    = 0b00111111;
        const IND  = 0b00100000;
        const TYPE = 0b00001111;

        const ALL = 0b11111111;
    }
}

pub enum IndexModes {
    ROff { reg : Regs, off : i8  },
    Rpp(Regs),
    Rd(Regs),
    Rdd(Regs),
    Rzero(Regs),
    RplusB(Regs),
    Rplus8(Regs),
    Rplus16(Regs),
    RplusA(Regs),
    RplusD(Regs),
    PCplus8,
    PCplus16,
    Ea,

    Dunno,
}

pub fn get_d(v : u8) -> u8{
    v & IndexedFlags::D.bits()
}

pub fn is_indirect(v : u8) -> bool {
    (v & IndexedFlags::IND.bits()) == IndexedFlags::IND.bits()
}

pub fn has_imm(v : u8) -> bool {
    (v & IndexedFlags::IMM.bits()) == IndexedFlags::IMM.bits()
}

pub fn get_reg(v : u8) -> Regs {
    match v & (IndexedFlags::R.bits()) {
        0b0000000 => Regs::X,
        0b0100000 => Regs::Y,
        0b1000000 => Regs::S,
        _ => Regs::U,
    }
}
pub fn get_index_type(v : u8) -> IndexModes {
    let v = v & IndexedFlags::TYPE.bits();
    
    let r = get_reg(v);

    match v {
        0b0000 => IndexModes::Rpp(r),
        0b0001 => IndexModes::Rd(r),
        0b0011 => IndexModes::Rdd(r),
        0b0100 => IndexModes::Rzero(r),
        0b0101 => IndexModes::RplusB(r),
        0b0110 => IndexModes::RplusA(r),
        0b1000 => IndexModes::Rplus8(r),
        0b1001 => IndexModes::Rplus16(r),
        0b1011 => IndexModes::RplusD(r),
        0b1100 => IndexModes::PCplus8,
        0b1101 => IndexModes::PCplus16,
        0b1111 => IndexModes::Ea,
        _ => IndexModes::Dunno,
    }
}

pub fn decode_imdex(mode : u8) -> IndexModes {
    IndexModes::Dunno
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

    let pf =  match *addr_mode {
        AddrModes::Illegal => format_illegal,
        AddrModes::Direct => format_direct,
        AddrModes::Inherent => format_inherent,
        AddrModes::Variant => format_variant,
        AddrModes::Relative => format_relative,
        AddrModes::Indexed => format_indexed,
        AddrModes::Immediate => format_immediate,
        AddrModes::Extended => format_extended,
    };

    pf(operand)
}


pub fn fetch_illegal(op_code : u16, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> (u16, u16 ) {
    (0,0)
}

pub fn fetch_direct(op_code : u16, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> (u16, u16 ) {
    (0, mem.load_byte(addr) as u16 )
}

pub fn fetch_inherent(op_code : u16, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> (u16, u16 ) {
    (0, 0 )
}

pub fn fetch_variant(op_code : u16, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> (u16, u16 ) {
    (0, 0 )
}

pub fn fetch_relative(op_code : u16, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> (u16, u16 ) {
    (0, 0 )
}

pub fn fetch_indexed(op_code : u16, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> (u16, u16 ) {
    (0, 0 )
}

pub fn fetch_immediate(op_code : u16, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> (u16, u16 ) {
    (0, mem.load_word(addr) )
}
pub fn fetch_extended(op_code : u16, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> (u16, u16 ) {
    (0, mem.load_word(addr) )
}

pub fn fetch_operand( addr_mode : &AddrModes,  op_code : u16, cpu : &mut Cpu, mem : &mut MemMap, addr : u16) -> (u16, u16 ) {

    let pf =  match *addr_mode {
        AddrModes::Illegal => fetch_illegal,
        AddrModes::Direct => fetch_direct,
        AddrModes::Inherent => fetch_inherent,
        AddrModes::Variant => fetch_variant,
        AddrModes::Relative => fetch_relative,
        AddrModes::Indexed => fetch_indexed,
        AddrModes::Immediate => fetch_immediate,
        AddrModes::Extended => fetch_extended,
    };

    pf(op_code, cpu,mem, addr)
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

