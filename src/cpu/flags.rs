
bitflags! {
    pub struct Flags: u8 {
        const E  = 1 << 7;
        const F  = 1 << 6;
        const H  = 1 << 5;
        const I  = 1 << 4;
        const N  = 1 << 3;
        const Z  = 1 << 2;
        const V  = 1 << 1;
        const C  = 1 << 0;
    }
}

#[inline]
fn test_n_b(val : u8) -> bool { val & 0x80 == 0x80 }
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
    pub fn set_flags( &mut self, flags: Flags, condition: bool ) {
        let status = self.bits;
        let new_status = if condition { status | flags.bits() } else { status & !flags.bits() };
        self.bits = new_status;
    }

    pub fn assign_flags(&mut self, val : u8) {
        // basically the ORCC instruction
        // doesn't affect the E flag

        let mut new_flags = Flags::new(val);
        new_flags.set(Flags::E, self.contains(Flags::E));
        *self = new_flags;
    }


    #[inline]
    pub fn test_8(&mut self, val : u8 ) {
        self.set(Flags::N, test_n_b(val));
        self.set(Flags::Z, test_z_b(val));
        self.set(Flags::V, false);

    }

    #[inline]
    pub fn test_16(&mut self, val : u16 ) {
        self.set(Flags::N, test_n_w(val));
        self.set(Flags::Z, test_z_w(val));
        self.set(Flags::V, false);
    }


}
