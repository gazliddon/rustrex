use mem::MemoryIO;
use cpu::{ Regs, InstructionDecoder, IndexedFlags, IndexModes};


pub trait AddressLines {
    #[inline(always)]
    fn ea<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        panic!("EA for {}", Self::name());
    }

    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8;
    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16;

    #[inline(always)]
    fn store_byte<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u8 ) -> u16;

    #[inline(always)]
    fn store_word<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u16 ) -> u16;

    #[inline(always)]
    fn fetch_byte_as_i16<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> i16 {
        let byte = Self::fetch_byte(mem,regs,ins) as i8;
        byte as i16
    }

    #[inline(always)]
    fn name() -> String;
}

////////////////////////////////////////////////////////////////////////////////
pub struct Direct { }

impl AddressLines for Direct {
    #[inline(always)]
    fn ea<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        let index = ins.fetch_byte(mem) as u16;
        regs.get_dp_ptr().wrapping_add(index)
    }

    fn name() -> String {
        "Direct".to_string()
    }

    #[inline(always)]
    fn fetch_byte<M: MemoryIO>( mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        mem.load_byte(Self::ea(mem,regs,ins))
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>( mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        mem.load_word(Self::ea(mem,regs,ins))
    }
    #[inline(always)]
    fn store_byte<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u8 ) -> u16{
        let ea = Self::ea(mem, regs, ins);
        mem.store_byte(ea,val);
        ea
    }

    #[inline(always)]
    fn store_word<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u16 )  -> u16{
        let ea = Self::ea(mem, regs, ins);
        mem.store_word(ea, val);
        ea
    }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Extended { }

impl AddressLines for Extended {
    #[inline(always)]
    fn ea<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        ins.fetch_word(mem)
    }

    fn name() -> String {
        "Extended".to_string()
    }

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
    #[inline(always)]
    fn store_byte<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u8 ) -> u16{
        let addr = Self::ea(mem,regs,ins);
        mem.store_byte(addr, val);
        addr
    }
    #[inline(always)]
    fn store_word<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u16 ) -> u16 {
        let addr = Self::ea(mem,regs,ins);
        mem.store_word(addr, val);
        addr
    }
}

////////////////////////////////////////////////////////////////////////////////
pub struct Immediate { }

impl AddressLines for Immediate {
    fn name() -> String {
        "Immediate".to_string()
    }
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        ins.fetch_byte(mem)
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        ins.fetch_word(mem)
    }

    #[inline(always)]
    fn store_byte<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u8 ) -> u16{
        panic!("tbd")
    }

    #[inline(always)]
    fn store_word<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u16 ) -> u16 {
        panic!("tbd")

    }
}

////////////////////////////////////////////////////////////////////////////////
pub struct Inherent { }

impl AddressLines for Inherent {
    fn name() -> String {
        "Inherent".to_string()
    }
    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        panic!("no")
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        panic!("no")
    }

    #[inline(always)]
    fn store_byte<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u8 ) -> u16{
        panic!("tbd")
    }

    #[inline(always)]
    fn store_word<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u16 ) -> u16 {
        panic!("tbd")

    }
}

////////////////////////////////////////////////////////////////////////////////
pub struct Indexed { }

impl AddressLines for Indexed {

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
                let addr = regs.get(&r);
                regs.incinc(&r);
                addr 
            },

            IndexModes::RSub(r) => {
                regs.dec(&r)
            },

            IndexModes::RSubSub(r) => {
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
                let v = ins.fetch_byte_as_i16(mem) as u16;
                regs.get(&r).wrapping_add(v)
            },

            IndexModes::RAddi16(r) => {
                // format!("{},{:?}",diss.fetch_word(mem) as i16, r)
                //
                let v = ins.fetch_word(mem);
                regs.get(&r).wrapping_add(v)
            },

            IndexModes::RAddD(r) => {
                // format!("D,{:?}", r) 
                let add_r = regs.get_d();
                regs.get(&r).wrapping_add(add_r)
            },

            IndexModes::PCAddi8 => {
                // format!("PC,{:?}",diss.fetch_byte(mem) as i8)
                let offset = ins.fetch_byte_as_i16(mem) as u16;
                regs.pc.wrapping_add(offset)
            },

            IndexModes::PCAddi16 => {
                // format!("PC,{:?}",diss.fetch_word(mem) as i16)
                let offset = ins.fetch_word(mem);
                regs.pc.wrapping_add(offset)
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
                regs.get(&r).wrapping_add((offset as i16) as u16)
            },

        };

        if index_mode.is_indirect() {
            mem.load_word(ea)
        } else {
            ea 
        }
    }
    fn name() -> String {
        "Indexed".to_string()
    }

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
    #[inline(always)]
    fn store_byte<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u8 ) -> u16{
        let ea = Self::ea(mem , regs , ins );
        mem.store_byte(ea, val);
        ea
    }

    #[inline(always)]
    fn store_word<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u16 ) -> u16 {
        let ea = Self::ea(mem , regs , ins );
        mem.store_word(ea, val);
        ea
    }
}

////////////////////////////////////////////////////////////////////////////////
pub struct Relative { }

impl AddressLines for Relative {
    fn name() -> String {
        "Relative".to_string()
    }

    #[inline(always)]
    fn fetch_byte<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        ins.fetch_byte(mem)
    }

    #[inline(always)]
    fn fetch_word<M: MemoryIO>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        ins.fetch_word(mem)
    }

    #[inline(always)]
    fn store_byte<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u8 ) -> u16{
        panic!("tbd")
    }

    #[inline(always)]
    fn store_word<M: MemoryIO>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u16 ) -> u16 {
        panic!("tbd")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait FetchWrite<M: MemoryIO> {
    fn fetch<A: AddressLines>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> Self;
    fn store<A: AddressLines>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : Self ) ;
}


impl<M : MemoryIO> FetchWrite<M> for u8 {

    fn fetch<A: AddressLines>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u8 {
        A::fetch_byte(mem, regs,ins)
    }

    fn store<A: AddressLines>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u8 )  {
        A::store_byte(mem , regs , ins , val ) ;
    }
}

impl<M : MemoryIO> FetchWrite<M> for u16 {

    fn fetch<A: AddressLines>(mem : &M, regs : &mut Regs, ins : &mut InstructionDecoder) -> u16 {
        A::fetch_word(mem, regs,ins)
    }

    fn store<A: AddressLines>(mem : &mut M, regs : &mut Regs, ins : &mut InstructionDecoder, val : u16 )  {
        A::store_word(mem , regs , ins , val ) ;
    }
}


