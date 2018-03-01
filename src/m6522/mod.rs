use mem::{ MemoryIO };
use sha1::Sha1;

// http://www.playvectrex.com/designit/chrissalo/via3.htm

#[repr(u16)]
pub enum Reg {
    PortA      = 0x0,
    PortB      = 0x1,
    DdrA       = 0x2,
    DdrB       = 0x3,
    T1CntL     = 0x4,
    T1CntH     = 0x5,

    T2CntL     = 0x6,
    T2CntH     = 0x7,

    T2Lo       = 0x8,
    T2Hi       = 0x9,

    ShiftReg   = 0xa,

    AuxCntl    = 0xb,

    Cnt1       = 0xc,
    IntFlags   = 0xd,
    IntEnable  = 0xe,
    PortANhs   = 0xf,
}

#[derive(Debug, Clone, Default)]

pub struct M6522 {
    regs : [u8; 16],
    start : u16,
    size :u16,
    last_byte : u16,
    name : String,
}


impl M6522 {

    pub fn new(start : u16, size : u16) -> Self {
        let last_byte = (size as u32 + start as u32) - 1;

        assert!(last_byte < 0x1_0000);

        Self {
            start,
            size,
            last_byte : last_byte as u16,
            name : format!("6522 : {:04x} {:04x}", start, size),
            .. Default::default()
        }
    }

    pub fn get_reg(&self, addr : u16) -> Reg {
        let reg_num = (addr - self.start) % self.size;
        let r: Reg = unsafe { ::std::mem::transmute(reg_num) };
        r
    }

}

impl MemoryIO for M6522 {

    fn get_range(&self) -> (u16, u16) {
        (self.start, self.last_byte)
    }

    fn update_sha1(&self, digest : &mut Sha1) {
        digest.update(&self.regs);
    }

    fn upload(&mut self, addr : u16, data : &[u8]) {
        panic!("tbd")
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn load_byte(&self, addr:u16) -> u8 {
        let reg = self.get_reg(addr);

        match reg {
            Reg::PortA       => {0} ,
            Reg::PortB       => {1} ,
            Reg::DdrA        => {2} ,
            Reg::DdrB        => {3} ,
            Reg::T1CntL      => {4} ,
            Reg::T1CntH      => {5} ,

            Reg::T2CntL      => {6} ,
            Reg::T2CntH      => {7} ,

            Reg::T2Lo        => {8} ,
            Reg::T2Hi        => {9} ,

            Reg::ShiftReg    => {10} ,

            Reg::AuxCntl     => {11} ,

            Reg::Cnt1        => {12} ,
            Reg::IntFlags    => {13} ,
            Reg::IntEnable   => {14} ,
            Reg::PortANhs    => {15} ,
        }
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        let reg = self.get_reg(addr);

        let dummy = match reg {
            Reg::PortA       => {0} ,
            Reg::PortB       => {1} ,
            Reg::DdrA        => {2} ,
            Reg::DdrB        => {3} ,
            Reg::T1CntL      => {4} ,
            Reg::T1CntH      => {5} ,

            Reg::T2CntL      => {6} ,
            Reg::T2CntH      => {7} ,

            Reg::T2Lo        => {8} ,
            Reg::T2Hi        => {9} ,

            Reg::ShiftReg    => {10} ,

            Reg::AuxCntl     => {11} ,

            Reg::Cnt1        => {12} ,
            Reg::IntFlags    => {13} ,
            Reg::IntEnable   => {14} ,
            Reg::PortANhs    => {15} ,
        };
    }
}

