use mem::MemoryIO;
use cpu::{ Regs, RegEnum, Flags, InstructionDecoder};
use cpu::{FetchWrite, AddressLines, Direct, Extended, Immediate, Inherent, Relative, Indexed};

use cpu::alu::{GazAlu};
use cpu::alu;

// use cpu::alu;

pub fn get_tfr_regs(op : u8) -> (RegEnum, RegEnum) {
    ( get_tfr_reg(op>>4), get_tfr_reg(op&0xf) )
}

fn get_tfr_reg(op : u8 ) -> RegEnum {

    match op {
        0 => RegEnum::D,
        1 => RegEnum::X,
        2 => RegEnum::Y,
        3 => RegEnum::U,
        4 => RegEnum::S,
        5 => RegEnum::PC,
        8 => RegEnum::A,
        9 => RegEnum::B,
        10 =>RegEnum::CC,
        11 =>RegEnum::DP,
        _ => {
            println!("op of {:02X}", op);
            panic!("illegal tfr regs")
        },
    }
}

// {{{
pub struct Cpu {
    pub regs: Regs,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: Regs::new(),
        }
    }

    pub fn from_regs(regs : Regs) ->  Cpu {
        Cpu {
            regs : regs,
        }
    }

}

////////////////////////////////////////////////////////////////////////////////

struct Alu<'a> {
    flags : &'a mut Flags,
}

impl Cpu {

    #[inline(always)]
    fn op8_2<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8, i0 : u8 ) -> u8{
        let i1 = A::fetch_byte(mem, &mut self.regs, ins) as u32;
        func(&mut self.regs.flags, write_mask, i0 as u32, i1)
    }
    fn op16_2<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u16, i0 : u16 ) -> u16 {
        let i1 = A::fetch_word(mem, &mut self.regs, ins) as u32;
        func(&mut self.regs.flags, write_mask, i0 as u32,i1)
    }

    #[inline(always)]
    fn opd_2<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u16 ) -> u16 {
        let i0 = self.regs.get_d() ;
        self.op16_2::<M,A>(mem,ins,write_mask, func, i0)
    }

    #[inline(always)]
    fn modd_2<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u16 ) {
        let r = self.opd_2::<M,A>(mem, ins, write_mask, func);
        self.regs.set_d(r);
    }


    #[inline(always)]
    fn opa_2<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) -> u8 {
        let i0 = self.regs.a as u32;
        let i1 = A::fetch_byte(mem, &mut self.regs, ins) as u32;
        func(&mut self.regs.flags, write_mask, i0,i1)
    }
    #[inline(always)]
    fn opb_2<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) -> u8{
        let i0 = self.regs.b as u32;
        let i1 = A::fetch_byte(mem, &mut self.regs, ins) as u32;
        func(&mut self.regs.flags, write_mask, i0,i1)
    }

    #[inline(always)]
    fn moda_2<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) {
        let r = self.opa_2::<M,A>(mem, ins, write_mask, func);
        self.regs.a = r;

    }
    #[inline(always)]
    fn modb_2<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) {
        let r = self.opb_2::<M,A>(mem,ins,write_mask, func);
        self.regs.b = r;
    }



    #[inline(always)]
    fn opa<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) -> u8{
        let i0 = self.regs.a as u32;
        let r = func(&mut self.regs.flags, write_mask, i0);
        r
    }

    #[inline(always)]
    fn opb<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) -> u8 {
        let i0 = self.regs.b as u32;
        let r = func(&mut self.regs.flags, write_mask, i0);
        r
    }

    #[inline(always)]
    fn moda<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) {
        let r = self.opa::<M,A>(mem,ins, write_mask, func);
        self.regs.a = r
    }

    #[inline(always)]
    fn modb<M: MemoryIO, A : AddressLines>( &mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) {
        let r = self.opb::<M,A>(mem,ins, write_mask, func);
        self.regs.b = r
    }

    #[inline(always)]
    fn rwmod8<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder, write_mask : u8, func : fn(&mut Flags, u8, u32) -> u8) {
        let ea = A::ea(mem, &mut self.regs, ins );
        let v = mem.load_byte(ea) as u32;
        let r = func(&mut self.regs.flags, write_mask, v );
        mem.store_byte(ea,r);
    }

    #[inline(always)]
    fn branch<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder, v : bool)  {

        let offset = A::fetch_byte_as_i16(mem, &mut self.regs, ins);

        if v {
            ins.next_addr = ins.next_addr.wrapping_add(offset as u16);
        }
    }

    #[inline(always)]
    fn lbranch<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder, v : bool)  {
        let offset = A::fetch_word(mem, &mut self.regs, ins);

        if v {
            ins.next_addr = ins.next_addr.wrapping_add(offset);
        }
    }

    #[inline(always)]
    fn post_clear(&mut self) {
        self.regs.flags.set(Flags::Z, true );
        self.regs.flags.set(Flags::N | Flags::V | Flags::C, false );
    }
}

// {{{ Todo next!
impl  Cpu {

    #[inline(always)]
    fn orcc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let v = A::fetch_byte(mem, &mut self.regs, ins);
        let cc = self.regs.flags.bits();
        self.regs.flags.set_flags(v | cc);
    }

    #[inline(always)]
    fn ldx<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let val =  A::fetch_word(mem, &mut self.regs, ins);
        self.regs.load_x(val);
    }

    #[inline(always)]
    fn stx<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let x = self.regs.x;
        A::store_word(mem, &mut self.regs, ins, x);
    }

    #[inline(always)]
    fn sta<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let r = self.regs.a;
        A::store_byte(mem, &mut self.regs, ins, r);
    }

    #[inline(always)]
    fn lda<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let v = A::fetch_byte(mem, &mut self.regs, ins);
        alu::nz::<u8>(&mut self.regs.flags, Flags::NZ.bits(), v as u32);
        self.regs.a = v
    }

    #[inline(always)]
    fn ldu<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let i0 =  A::fetch_word(mem, &mut self.regs, ins);
        alu::nz::<u16>(&mut self.regs.flags, Flags::NZ.bits(), i0 as u32);
        self.regs.u = i0;
    }

    fn lsla_asla<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.moda::<M,A>(mem,ins, Flags::NZVC.bits(), u8::asl);
    }

    fn lslb_aslb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modb::<M,A>(mem,ins, Flags::NZVC.bits(), u8::asl);
    }

    fn asra<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.moda::<M,A>(mem,ins, Flags::NZVC.bits(), u8::asr);
    }
    fn asrb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modb::<M,A>(mem,ins, Flags::NZVC.bits(), u8::asr);
    }

    fn asr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.rwmod8::<M,A>(mem,  ins, Flags::NZVC.bits(), u8::asr);
    }

    #[inline(always)]
    fn tfr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let operand = ins.fetch_byte(mem); 
        let (a,b) = get_tfr_regs(operand as u8);
        let av = self.regs.get(&a);
        self.regs.set(&b, av);
    }

    #[inline(always)]
    fn lds<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let v =  u16::fetch::<A>(mem, &mut self.regs, ins);
        self.regs.load_s(v);
    }

    #[inline(always)]
    fn abx<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let x = self.regs.x;
        self.regs.x = x.wrapping_add(self.regs.b as u16);
    }

    #[inline(always)]
    fn sts<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let r = self.regs.s;
        A::store_word(mem, &mut self.regs, ins, r);
        self.regs.flags.test_16(r)
    }


    fn beq<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let z = self.regs.flags.contains(Flags::Z);
        self.branch::<M,A>(mem,ins,z);
    }

    fn bge<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.ge();
        self.branch::<M,A>(mem,ins,cond);
    }

    #[inline(always)]
    fn bgt<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.gt();
        self.branch::<M,A>(mem,ins,cond);
    }

    #[inline(always)]
    fn blo_bcs<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::C);
        self.branch::<M,A>(mem,ins,cond);
    }

    #[inline(always)]
    fn brn<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::N);
        self.branch::<M,A>(mem,ins,cond);
    }

    #[inline(always)]
    fn bhs_bcc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::C);
        self.branch::<M,A>(mem,ins,!cond);
    }

    fn bhi<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.hi();
        self.branch::<M,A>(mem,ins,cond);
    }

    fn ble<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.le();
        self.branch::<M,A>(mem,ins,cond);
    }

    fn bls<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.ls();
        self.branch::<M,A>(mem,ins,cond);
    }

    fn blt<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.lt();
        self.branch::<M,A>(mem,ins,cond);
    }

    fn pushu_byte<M: MemoryIO>(&mut self, mem : &mut M, ins : &mut InstructionDecoder, v : u8) {
        let s = self.regs.s.wrapping_sub(1);
        mem.store_byte(s,v);
        self.regs.s = s;
    }

    fn pushu_word<M: MemoryIO>(&mut self, mem : &mut M, ins : &mut InstructionDecoder, v : u16) {

        let mut s = self.regs.s.wrapping_sub(1);
        mem.store_byte(s,v as u8);
        s = s.wrapping_sub(1);
        mem.store_byte(s,(v >> 8) as u8);
        self.regs.s = s;
    }

    fn popu_byte<M: MemoryIO>(&mut self, mem : &mut M, ins : &mut InstructionDecoder ) -> u8 {
        let mut s = self.regs.s;
        let r = mem.load_byte(s);
        s = s.wrapping_add(1);
        self.regs.s = s;
        r
    }

    fn popu_word<M: MemoryIO>(&mut self, mem : &mut M, ins : &mut InstructionDecoder ) -> u16 {
        let w = mem.load_word(self.regs.s);
        self.regs.s = self.regs.s.wrapping_add(2);
        w
    }

    fn rts<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        ins.next_addr = self.popu_word::<M>(mem, ins);
    }

    fn bsr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {

        let offset = A::fetch_byte_as_i16(mem, &mut self.regs, ins) as u16;

        let next_op = ins.next_addr;

        self.pushu_word(mem, ins, next_op);

        ins.next_addr = ins.next_addr.wrapping_add( offset );
    }

    fn bvc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = !self.regs.flags.contains(Flags::V);
        self.branch::<M,A>(mem,ins,cond);
    }

    fn bne<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = !self.regs.flags.contains(Flags::Z);
        self.branch::<M,A>(mem,ins,cond);
    }

    fn bvs<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::V);
        self.branch::<M,A>(mem,ins,cond);
    }

    fn bmi<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::N);
        self.branch::<M,A>(mem,ins,cond);
    }

    fn bra<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.branch::<M,A>(mem,ins,true);
    }

    fn bpl<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = !self.regs.flags.contains(Flags::N);
        self.branch::<M,A>(mem,ins,cond);
    }


    fn andcc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let i0 = self.regs.flags.bits() as u32;
        let i1 = u8::fetch::<A>(mem, &mut self.regs, ins) as u32;

        let new_f = u8::and(&mut self.regs.flags, 0, i0, i1);
        self.regs.flags.set_flags(new_f);
    }

    fn lsl_asl<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.rwmod8::<M,A>(mem, ins, Flags::NZC.bits(), u8::asl);
    }

    //////////////////////////////////////////////////////////////////////////////// 

    #[inline(always)]
    fn adda<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.moda_2::<M,A>(mem,ins, Flags::NZVCH.bits(), u8::add);
    }

    #[inline(always)]
    fn adca<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.moda_2::<M,A>(mem,ins, Flags::NZVCH.bits(), u8::adc);
    }

    #[inline(always)]
    fn adcb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modb_2::<M,A>(mem,ins, Flags::NZVCH.bits(), u8::adc);
    }


    #[inline(always)]
    fn addb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modb_2::<M,A>(mem,ins, Flags::NZVCH.bits(), u8::add);
    }

    #[inline(always)]
    fn addd<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modd_2::<M,A>(mem,ins,Flags::NZVC.bits(), u16::add);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    #[inline(always)]
    fn anda<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.moda_2::<M,A>(mem,ins, Flags::NZVC.bits(), u8::and);
    }

    #[inline(always)]
    fn andb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modb_2::<M,A>(mem,ins, Flags::NZVC.bits(), u8::and);
    }

    #[inline(always)]
    fn bita<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.opa_2::<M,A>(mem,ins,Flags::NZVC.bits(), u8::and);
    }

    #[inline(always)]
    fn bitb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.opb_2::<M,A>(mem,ins,Flags::NZVC.bits(), u8::and);
    }

    //////////////////////////////////////////////////////////////////////////////// 

    #[inline(always)]
    fn clra<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.regs.a = 0;
        self.post_clear();
    }

    #[inline(always)]
    fn clrb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.regs.b = 0;
        self.post_clear();
    }

    #[inline(always)]
    fn clr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        A::store_byte(mem, &mut self.regs, ins, 0);
        self.post_clear();
    }

    //////////////////////////////////////////////////////////////////////////////// 
    #[inline(always)]
    fn cmpa<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.opa_2::<M,A>(mem,ins,Flags::NZVC.bits(), u8::sub);
    }

    #[inline(always)]
    fn cmpb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.opb_2::<M,A>(mem,ins,Flags::NZVC.bits(), u8::sub);
    }

    #[inline(always)]
    fn cmpd<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let i0 = self.regs.get_d();
        self.op16_2::<M,A>(mem,ins,Flags::NZVC.bits(), u16::sub, i0);
    }

    #[inline(always)]
    fn cmpu<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let i0 = self.regs.u;
        self.op16_2::<M,A>(mem,ins, Flags::NZVC.bits(), u16::sub, i0);
    }

    #[inline(always)]
    fn cmpx<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let i0 = self.regs.x;
        self.op16_2::<M,A>(mem,ins, Flags::NZVC.bits(), u16::sub, i0);
    }

    #[inline(always)]
    fn cmpy<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let i0 = self.regs.y;
        self.op16_2::<M,A>(mem,ins, Flags::NZVC.bits(), u16::sub, i0);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn coma<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.moda::<M,A>(mem,ins, Flags::NZVC.bits(), u8::com);
    }

    fn comb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modb::<M,A>(mem,ins, Flags::NZVC.bits(), u8::com);
    }

    fn com<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.rwmod8::<M,A>(mem,  ins, Flags::NZVC.bits(), u8::com);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn deca<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.moda::<M,A>(mem,ins, Flags::NZV.bits(), u8::dec);
    }

    fn decb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modb::<M,A>(mem,ins, Flags::NZV.bits(), u8::dec);
    }

    fn dec<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.rwmod8::<M,A>(mem,  ins, Flags::NZV.bits(), u8::dec);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn inca<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        println!("about to inca");
        self.moda::<M,A>(mem,ins, Flags::NZV.bits(), u8::inc);
    }
    fn incb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modb::<M,A>(mem,ins, Flags::NZV.bits(), u8::inc);
    }
    fn inc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.rwmod8::<M,A>(mem,  ins, Flags::NZV.bits(), u8::inc);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn eora<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.moda_2::<M,A>(mem,ins, Flags::NZV.bits(), u8::eor);
    }
    fn eorb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.modb_2::<M,A>(mem,ins, Flags::NZV.bits(), u8::eor);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn daa<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        // fuck sakes
        let a = self.regs.a as u32;

        let msn = a & 0xf0;
        let lsn = a & 0xf0;

        let mut cf= 0u32;

        if lsn > 0x09 || self.regs.flags.contains(Flags::H) {
            cf |= 0x06;
        }

        if msn > 0x80 && lsn >0x09 {
            cf |= 0x60;
        }
    
        if msn > 0x90 || self.regs.flags.contains(Flags::C) {
            cf |= 0x60;
        }

        let temp = cf.wrapping_add(a);

        self.regs.flags.set(Flags::C, temp & 0x100 != 0 );
        self.regs.flags.set(Flags::V  | Flags::N, false);


        let new_a = alu::nz::<u8>(&mut self.regs.flags,Flags::NZ.bits(), temp);

        self.regs.a = new_a

    }

    fn exg<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let operand = ins.fetch_byte(mem); 
        let (a,b) = get_tfr_regs(operand as u8);
        let temp = self.regs.get(&b);
        let av = self.regs.get(&a);
        let bv = self.regs.get(&b);
        self.regs.set(&b, av);
        self.regs.set(&a, bv)
    }

////////////////////////////////////////////////////////////////////////////////
    fn lbrn<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::N);
        self.lbranch::<M,A>(mem,ins,cond);
    }
    fn lbhi<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.hi();
        self.lbranch::<M,A>(mem,ins,cond);
    }

    fn lbra<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.lbranch::<M,A>(mem,ins,true);
    }

    fn lbsr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let offset = A::fetch_word(mem, &mut self.regs, ins) as u16;
        let next_op = ins.next_addr;
        self.pushu_word(mem, ins, next_op);
        ins.next_addr = ins.next_addr.wrapping_add( offset );
    }

    fn lbls<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.ls();
        self.lbranch::<M,A>(mem,ins,cond);
    }

    fn lbge<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.ge();
        self.lbranch::<M,A>(mem,ins,cond);
    }
    fn lblt<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.lt();
        self.lbranch::<M,A>(mem,ins,cond);
    }
    fn lbgt<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.gt();
        self.lbranch::<M,A>(mem,ins,cond);
    }

    fn lbvc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = !self.regs.flags.contains(Flags::V);
        self.lbranch::<M,A>(mem,ins,cond);
    }
    fn lbvs<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::V);
        self.lbranch::<M,A>(mem,ins,cond);
    }
    fn lbpl<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = !self.regs.flags.contains(Flags::N);
        self.lbranch::<M,A>(mem,ins,cond);
    }
    fn lbmi<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::N);
        self.lbranch::<M,A>(mem,ins,cond);
    }
    fn lbhs_lbcc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::C);
        self.lbranch::<M,A>(mem,ins,!cond);
    }

    fn lblo_lbcs<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.contains(Flags::C);
        self.lbranch::<M,A>(mem,ins,cond);
    }

    fn lbne<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = !self.regs.flags.contains(Flags::Z);
        self.lbranch::<M,A>(mem,ins,cond);
    }
    fn lbeq<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let z = self.regs.flags.contains(Flags::Z);
        self.lbranch::<M,A>(mem,ins,z);
    }
////////////////////////////////////////////////////////////////////////////////
}
// }}}

// {{{ Op Codes
impl  Cpu {

    fn cwai<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("cwai NO!")
    }

    fn jmp<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("jmp NO!")
    }
    fn jsr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("jsr NO!")
    }

    fn ldb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("ldb NO!")
    }
    fn ldd<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("ldd NO!")
    }
    fn leas<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("leas NO!")
    }
    fn leau<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("leau NO!")
    }
    fn leax<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("leax NO!")
    }
    fn leay<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("leay NO!")
    }
    fn lsr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lsr NO!")
    }
    fn lsra<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lsra NO!")
    }
    fn lsrb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lsrb NO!")
    }
    fn mul<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("mul NO!")
    }
    fn neg<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("neg NO!")
    }
    fn nega<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("nega NO!")
    }
    fn negb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("negb NO!")
    }
    fn nop<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("nop NO!")
    }
    fn ora<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("ora NO!")
    }
    fn orb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("orb NO!")
    }
    fn pshs<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("pshs NO!")
    }
    fn pshu<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("pshu NO!")
    }
    fn puls<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("puls NO!")
    }
    fn pulu<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("pulu NO!")
    }
    fn reset<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("reset NO!")
    }
    fn rol<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("rol NO!")
    }
    fn rola<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("rola NO!")
    }
    fn rolb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("rolb NO!")
    }
    fn ror<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("ror NO!")
    }
    fn rora<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("rora NO!")
    }
    fn rorb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("rorb NO!")
    }
    fn rti<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("rti NO!")
    }
    fn sbca<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("sbca NO!")
    }
    fn sbcb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("sbcb NO!")
    }
    fn sex<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("sex NO!")
    }
    fn stb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("stb NO!")
    }
    fn std<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("std NO!")
    }
    fn stu<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("stu NO!")
    }
    fn suba<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("suba NO!")
    }
    fn subb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("subb NO!")
    }
    fn subd<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("subd NO!")
    }
    fn swi<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("swi NO!")
    }
    fn sync<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("sync NO!")
    }
    fn tst<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("tst NO!")
    }
    fn tsta<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("tsta NO!")
    }
    fn tstb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("tstb NO!")
    }
    fn swi3<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("swi3 NO!")
    }
    fn cmps<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("cmps NO!")
    }

    fn swi2<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("swi2 NO!")
    }
    fn ldy<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("ldy NO!")
    }
    fn lble<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let cond = self.regs.flags.le();
        self.lbranch::<M,A>(mem,ins,cond);
    }
    fn sty<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("sty NO!")
    }

    fn unimplemented(&mut self, ins : &mut InstructionDecoder) {
        panic!("unimplemnted op code")
    }

    fn get_pc(&self) -> u16 {
        self.regs.pc
    }

    pub fn step<M: MemoryIO>(&mut self, mem : &mut M) -> InstructionDecoder {

        {
            let alu = Alu {
                flags : &mut self.regs.flags

            };
        }

        let mut ins = InstructionDecoder::new(self.get_pc());

        let op = ins.fetch_instruction(mem);

        macro_rules! handle_op {
            ($addr:ident, $action:ident) => (
                { self.$action::<M, $addr>(mem, &mut ins); }) }

        op_table!(op, { self.unimplemented(&mut ins)});

        self.regs.pc = ins.next_addr;

        ins
    }
}

//
// }}}

