
bitflags! {
    pub struct Flags: u8 {
        const E  = 0b00000001;
        const F  = 0b00000010;
        const H  = 0b00000100;
        const I  = 0b00001000;
        const N  = 0b00010000;
        const Z  = 0b00100000;
        const V  = 0b01000000;
        const C  = 0b10000000;
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
