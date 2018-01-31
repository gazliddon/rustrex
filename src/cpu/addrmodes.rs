use mem::MemoryIO;
use cpu::{ Regs, InstructionDecoder, IndexedFlags, IndexModes };

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

impl Indexed {

    fn ea<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {

        let index_mode_id = ins.fetch_byte(mem);
        let index_mode = IndexedFlags::new(index_mode_id) ;

        let ea = match index_mode.get_index_type() {

            IndexModes::RPlus(r) => { 
                // format!(",{:?}+",r)
                let addr = regs.get(&r);
                regs.inc(&r);
                addr 
            },

            IndexModes::RPlusPlus(r) => {
                // format!(",{:?}++",r)
                let addr = regs.get(&r);
                regs.incinc(&r);
                addr 
            },

            IndexModes::RSub(r) => {
                regs.dec(&r)
            },

            IndexModes::RSubSub(r) =>{
                regs.decdec(&r)
            },

            IndexModes::RZero(r) => { 
                regs.get(&r)
            },

            IndexModes::RAddB(r) => { 
                // format!("B,{:?}", r) 
                let add_r = regs.b as u16;
                regs.get(&r).wrapping_add(add_r)
            },

            IndexModes::RAddA(r) => {
                // format!("A,{:?}", r) 
                let add_r = regs.a as u16;
                regs.get(&r).wrapping_add(add_r)
            },

            IndexModes::RAddi8(r) => {
                // format!("{},{:?}",diss.fetch_byte(mem) as i8, r)
                panic!("IndexModes::RAddi8(r)")
            },

            IndexModes::RAddi16(r) => {
                // format!("{},{:?}",diss.fetch_word(mem) as i16, r)
                panic!("IndexModes::RAddi16(r)")
            },

            IndexModes::RAddD(r) => {
                // format!("D,{:?}", r) 
                let add_r = regs.get_d();
                regs.get(&r).wrapping_add(add_r)
            },

            IndexModes::PCAddi8 => {
                // format!("PC,{:?}",diss.fetch_byte(mem) as i8)
                panic!("IndexModes::PCAddi8");
            },

            IndexModes::PCAddi16 => {
                panic!("IndexModes::PCAddi16")
                // format!("PC,{:?}",diss.fetch_word(mem) as i16)
            },

            IndexModes::Illegal => { 
                // "illegal".to_string() 
                panic!("IndexModes::Illegal")
            },

            IndexModes::Ea=> {
                // format!("0x{:04X}", diss.fetch_word(mem))
                ins.fetch_word(mem)
            },

            IndexModes::ROff(r,offset)=> {
                // format!("{}, {:?}", offset, r) 
                // regs.get(&r).wrapping_add(offset);
                panic!("IndexModes::ROff(r,offset)=>")
            },

        };

        if index_mode.is_indirect() {

        };

        // ea 
        panic!("final bit of ea calc todo");
    }
}

impl AddressLines for Indexed {
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        let ea = Self::ea(mem , regs , ins );
        mem.load_byte(ea)
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        let ea = Self::ea(mem , regs , ins );
        mem.load_word(ea)
    }
}

////////////////////////////////////////////////////////////////////////////////
pub struct Relative { }

impl AddressLines for Relative {
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        ins.fetch_byte(mem);
        panic!("Relative byte")
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        ins.fetch_word(mem);
        panic!("Relative word")
    }

}

////////////////////////////////////////////////////////////////////////////////


