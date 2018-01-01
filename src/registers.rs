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

impl Flags {
    pub fn new(val : u8) -> Flags {
        Flags {
            bits: val
        }
    }

    pub fn test_8(&mut self, val : u8 ) {
        self.set(Flags::N, (val&0x80 == 0x80));
        self.set(Flags::Z, val == 0);
        self.set(Flags::V, false);
    }

    pub fn test_16(&mut self, val : u16 ) {
        self.set(Flags::N, (val&0x80 == 0x8000));
        self.set(Flags::Z, val == 0);
        self.set(Flags::V, false);
    }


}

#[derive(Debug)]
pub struct Regs {
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
    fn get_d(&self) -> u16 { ( ( self.a as u16 ) << 8 ) | self.b as u16 }
    fn set_d(&mut self, d : u16) { self.a = (d >> 8) as u8; self.b = d as u8; }

    pub fn new() -> Regs {
        Regs {
            a : 0,
            b : 0,
            x : 0,
            y : 0,
            u : 0,
            s : 0,
            pc: 0,
            dp: 0,
            flags: Flags::new(0),
        }
    }

    pub fn load_a(&mut self, val : u8) {
        self.flags.test_8(val);
        self.a = val;
    }

    pub fn load_b(&mut self, val : u8) {
        self.flags.test_8(val);
        self.b = val;
    }

    pub fn load_d(&mut self, val : u16) {
        self.flags.test_16(val);
        self.set_d(val)
    }

    pub fn load_x(&mut self, val : u16) {
        self.flags.test_16(val);
        self.x = val
    }

    pub fn load_y(&mut self, val : u16) {
        self.flags.test_16(val);
        self.y = val
    }

    pub fn load_s(&mut self, val : u16) {
        self.flags.test_16(val);
        self.s = val
    }

    pub fn load_u(&mut self, val : u16) {
        self.flags.test_16(val);
        self.u = val
    }
}

