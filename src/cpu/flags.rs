
bitflags! {

#[derive( Default, Serialize, Deserialize)]
    pub struct Flags: u8 {

        const E  = 1 << 7;
        const F  = 1 << 6;
        const H  = 1 << 5;
        const I  = 1 << 4;
        const N  = 1 << 3;
        const Z  = 1 << 2;
        const V  = 1 << 1;
        const C  = 1 ;

        const NZVCH = 
            Self::N.bits |
            Self::Z.bits |
            Self::V.bits |
            Self::C.bits |
            Self::H.bits ;

        const NZVC = 
            Self::N.bits |
            Self::Z.bits |
            Self::V.bits |
            Self::C.bits ;

        const NZC = Self::N.bits | Self::Z.bits | Self::C.bits ;
        const NZ  = Self::N.bits | Self::Z.bits ;
        const NZV = Self::N.bits | Self::Z.bits | Self::V.bits ;
    }
}

// #[inline]
// fn test_n_b(val : u8) -> bool { val & 0x80 == 0x80 }
#[inline]
fn test_z_b(val : u8) -> bool { val == 0 }
#[inline]
fn test_n_w(val : u16) -> bool { val & 0x8000 == 0x8000 }
#[inline]
fn test_z_w(val : u16) -> bool { val == 0 }

impl Flags {
    pub fn new(val : u8) -> Flags {
        Flags {
            bits: val
        }
    }

    #[inline]
    pub fn set_flags(&mut self, val : u8) {
        self.bits = val
    }

    pub fn set_w_mask(&mut self, mask : u8, val : u8) {
        self.bits = (self.bits & !mask) | (val & mask)
    }


    #[inline]
    pub fn test_16(&mut self, val : u16 ) {
        self.set(Flags::N, test_n_w(val));
        self.set(Flags::Z, test_z_w(val));
    }

    pub fn le(&self) -> bool {
        let v = self.contains(Flags::V);
        let n = self.contains(Flags::N);
        let z = self.contains(Flags::Z);
        z | (v^n)
    }

    pub fn gt(&self) -> bool {
        !self.le()
    }
    pub fn lt(&self) -> bool {
        !self.ge()
    }

    pub fn ge(&self) -> bool {
        let v = self.contains(Flags::V);
        let n = self.contains(Flags::N);
        !(v ^ n)
    }

    pub fn hi(&self) -> bool {
        let c = self.contains(Flags::C);
        let z = self.contains(Flags::Z);
        !(c | z)
    }

    pub fn ls(&self) -> bool {
        !self.hi()
    }


}
