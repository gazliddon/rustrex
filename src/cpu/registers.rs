use cpu::Flags;

#[derive(Debug)]
pub enum RegEnum {
    A, B, X, Y, U, S, D, DP, CC, PC
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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

    pub fn set(&mut self, r : &RegEnum, val : u16)  {
        match *r {
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

    pub fn get(&self, r: &RegEnum) -> u16 {
        match *r {
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

    #[inline(always)]
    pub fn wrapping_add_and_set(&mut self, r : &RegEnum, v : u16) -> u16{
        let mut rv = self.get(r);
        rv = rv.wrapping_add(v);
        self.set(r, rv);
        rv
    }


    #[inline(always)]
    pub fn inc(&mut self, r: &RegEnum) -> u16 {
        self.wrapping_add_and_set(r, 1)
    }

    #[inline(always)]
    pub fn incinc(&mut self, r: &RegEnum) -> u16{
        let ret = self.wrapping_add_and_set(r, 2);
        ret
    }

    #[inline(always)]
    pub fn dec(&mut self, r: &RegEnum) -> u16{
        self.wrapping_add_and_set(r, 0xffff)
    }

    #[inline(always)]
    pub fn decdec(&mut self, r: &RegEnum) -> u16{
        self.wrapping_add_and_set(r, 0xfffe)
    }

    #[inline(always)]
    pub fn get_dp_ptr(&self) -> u16 {
        let dp = (self.dp as u16) << 8;
        dp
    }

    #[inline(always)]
    pub fn get_d(&self) -> u16 {
        ( ( self.a as u16 ) << 8 ) | self.b as u16 
    }

    #[inline(always)]
    pub fn set_d(&mut self, d : u16) {
        self.a = (d >> 8) as u8; self.b = d as u8; 
    }

    pub fn new() -> Regs {
        Regs {
            a : 0, b : 0, x : 0, y : 0, u : 0, s : 0, pc: 0, dp: 0,
            flags: Flags::new(0),
        }
    }

    pub fn load_d(&mut self, val : u16) {
        self.flags.test_16(val);
        self.set_d(val)
    }



    pub fn clear_c(&mut self) {
        self.flags.set(Flags::C, false);
    }

    pub fn get_c(&self) -> u8 {
        if self.flags.contains(Flags::C) {
            1
        } else {
            0
        }
    }
}

