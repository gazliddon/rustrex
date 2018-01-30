use mem::MemoryIO;
use cpu::{ Regs, InstructionDecoder };

pub trait AddressLines {
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8;
    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16;
}

////////////////////////////////////////////////////////////////////////////////
pub struct Direct { }

impl Direct {

    #[inline(always)]
    fn ea<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        let index = ins.fetch_byte(mem) as u16;
        regs.get_dp_ptr().wrapping_add(index)
    }
}

impl AddressLines for Direct {
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>( mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        mem.load_byte(Self::ea(mem,regs,ins))
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>( mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        mem.load_word(Self::ea(mem,regs,ins))
    }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Extended { }

impl Extended {
    #[inline(always)]
    fn ea<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        ins.fetch_word(mem)
    }
}

impl AddressLines for Extended {
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>( mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        let addr = Self::ea(mem,regs,ins);
        mem.load_byte(addr)
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        let addr = Self::ea(mem,regs,ins);
        mem.load_word(addr)
    }
}

////////////////////////////////////////////////////////////////////////////////
pub struct Immediate { }

impl AddressLines for Immediate {
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        ins.fetch_byte(mem)
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        ins.fetch_word(mem)
    }

}

////////////////////////////////////////////////////////////////////////////////
pub struct Inherent { }

impl AddressLines for Inherent {
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        panic!("no")
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        panic!("no")
    }

}

////////////////////////////////////////////////////////////////////////////////
pub struct Indexed { }

impl AddressLines for Indexed {
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        panic!("no")
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        panic!("no")
    }
}

////////////////////////////////////////////////////////////////////////////////
pub struct Relative { }

impl AddressLines for Relative {
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        ins.fetch_byte(mem)
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        ins.fetch_word(mem)
    }

}

////////////////////////////////////////////////////////////////////////////////


