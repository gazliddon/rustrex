use cpu::RegEnum;


#[derive(Debug)]
pub enum IndexModes {

    ROff(RegEnum,u16),

    RPlus(RegEnum),     //               ,R+              2 0 |
    RPlusPlus(RegEnum), //               ,R++             3 0 |
    RSub(RegEnum),      //               ,-R              2 0 |
    RSubSub(RegEnum),   //               ,--R             3 0 |
    RZero(RegEnum),     //               ,R               0 0 |
    RAddB(RegEnum),     //             (+/- B),R          1 0 |
    RAddA(RegEnum),     //             (+/- A),R          1 0 |
    RAddi8(RegEnum),    //    (+/- 7 b  it offset),R      1 1 |
    RAddi16(RegEnum),   //      (+/- 15 bit offset),R     4 2 |
    RAddD(RegEnum),     //             (+/- D),R          4 0 |
    PCAddi8,         //      (+/- 7 bit offset),PC     1 1 |
    PCAddi16,        //      (+/- 15 bit offset),PC    5 2 |
    Illegal,         //              Illegal           u u |
    Ea,
}

bitflags! {
    pub struct IndexedFlags: u8 {
        const NOT_IMM     = 0b_1000_0000;
        const R           = 0b_0110_0000;
        const D           = 0b_0011_1111;
        const OFFSET      = 0b_0001_1111;
        const OFFSET_SIGN = 0b_0001_0000;
        const IND         = 1 << 4;
        const TYPE        = 0b_0000_1111;
        const IS_EA       = 0b_1001_1111;
    }
}

impl IndexedFlags {

    fn get_offset(&self) -> u16 {
        let mut v = ( self.bits & IndexedFlags::OFFSET.bits() ) as u16;

        if self.contains(Self::OFFSET_SIGN) {
            v |= 0xfff0
        }
        v
    }

    pub fn new(val : u8) -> Self {
        IndexedFlags {
            bits: val
        }
    }

    pub fn is_ea(&self) -> bool {
        self.bits == IndexedFlags::IS_EA.bits()
    }

    pub fn is_indirect(&self) -> bool {
        self.contains(Self::IND | Self::NOT_IMM)
    }

    fn not_imm(&self) -> bool {
        self.contains(Self::NOT_IMM)
    }

    fn get_reg(&self) -> RegEnum {
        match ( self.bits & (IndexedFlags::R.bits()) ) >> 5{
            0 => RegEnum::X,
            1 => RegEnum::Y,
            2 => RegEnum::U,
            _ => RegEnum::S,
        }
    }

    pub fn get_index_type(&self) -> IndexModes {

        let r = self.get_reg();

        if self.is_ea() {
            return IndexModes::Ea
        }

        if self.not_imm() {

            let index_type = self.bits & IndexedFlags::TYPE.bits();

            return match index_type {
                0b0000 => IndexModes::RPlus(r),     //               ,R+              2 0 |
                0b0001 => IndexModes::RPlusPlus(r), //               ,R++             3 0 |
                0b0010 => IndexModes::RSub(r),      //               ,-R              2 0 |
                0b0011 => IndexModes::RSubSub(r),   //               ,--R             3 0 |
                0b0100 => IndexModes::RZero(r),     //               ,R               0 0 |
                0b0101 => IndexModes::RAddB(r),     //             (+/- B),R          1 0 |
                0b0110 => IndexModes::RAddA(r),     //             (+/- A),R          1 0 |
                // 0b0111 => IndexModes::Illegal,      //              Illegal           u u |
                0b1000 => IndexModes::RAddi8(r),    //    (+/- 7 b  it offset),R      1 1 |
                0b1001 => IndexModes::RAddi16(r),   //      (+/- 15 bit offset),R     4 2 |
                // 0b1010 => IndexModes::Illegal,      //              Illegal           u u |
                0b1011 => IndexModes::RAddD(r),     //             (+/- D),R          4 0 |
                0b1100 => IndexModes::PCAddi8,      //      (+/- 7 bit offset),PC     1 1 |
                0b1101 => IndexModes::PCAddi16,     //      (+/- 15 bit offset),PC    5 2 |
                // 0b1110 => IndexModes::Illegal,      //              Illegal           u u |
                _ => IndexModes::Illegal,
            }
        }

        IndexModes::ROff(r, self.get_offset())
    }
}





