use cpu::Flags;

#[derive(Debug)]
pub enum RegEnum {
    A, B, X, Y, U, S, D, DP, CC, PC
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

    pub fn set(&mut self, r : RegEnum, val : u16)  {
        match r {
            RegEnum::A => self.a = val as u8,
            RegEnum::B => self.b = val as u8,
            RegEnum::X => self.x = val,
            RegEnum::Y => self.y = val,
            RegEnum::U => self.u = val,
            RegEnum::S => self.s = val, 
            RegEnum::D => self.set_d(val),
            RegEnum::DP => self.dp = val as u8,
            RegEnum::CC => self.flags = Flags::new(val as u8),
            RegEnum::PC => self.pc = val,
        } 
    }

    pub fn get(&self, r: RegEnum) -> u16 {
        match r {
            RegEnum::A => self.a as u16,
            RegEnum::B => self.b as u16,
            RegEnum::X => self.x,
            RegEnum::Y => self.y,
            RegEnum::U => self.u,
            RegEnum::S => self.s,
            RegEnum::D => self.get_d(),
            RegEnum::DP => self.dp as u16,
            RegEnum::CC => self.flags.bits() as u16,
            RegEnum::PC => self.pc,
        } 
    }

    pub fn get_dp_ptr(&self) -> u16 {
        let dp = (self.dp as u16) << 8;
        dp
    }

    pub fn get_d(&self) -> u16 {
        ( ( self.a as u16 ) << 8 ) | self.b as u16 
    }

    pub fn set_d(&mut self, d : u16) {
        self.a = (d >> 8) as u8; self.b = d as u8; 
    }

    pub fn new() -> Regs {
        Regs {
            a : 0, b : 0, x : 0, y : 0, u : 0, s : 0, pc: 0, dp: 0,
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

