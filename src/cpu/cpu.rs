use mem::MemoryIO;
use cpu::{ Regs, RegEnum, Flags, InstructionDecoder};
use cpu::{FetchWrite, AddressLines, Direct, Extended, Immediate, Inherent, Relative, Indexed};


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

extern crate num;

pub trait GazAlu : num::PrimInt + num::Unsigned + num::traits::WrappingAdd + num::traits::WrappingMul {

    fn hi_bit_mask() -> Self;
    fn half_bit_mask() -> Self;

    fn get_n<N: GazAlu>(a : N) -> bool {
        (a & N::hi_bit_mask()) == N::hi_bit_mask()
    }

    fn get_overflow<N : GazAlu>(a : N, b : N, r :N) -> bool {

        if Self::get_n(a) == Self::get_n(b) {
            Self::get_n(a) != Self::get_n(r)
        } else {
            false
        }

        // Self::get_n((!(a^b))&r)
    }

    fn one_zero(f : bool) -> Self {
        if f { Self::one() } else { Self::zero() }
    }
    fn true_false(v : Self) -> bool {
        v != Self::zero()
    }

    fn adc(f : &mut Flags, a : Self, b : Self  ) -> Self {

        let c_val = Self::one_zero(f.contains(Flags::C));

        let r = a.wrapping_add( &b.wrapping_add( &c_val ) );

        let h = Self::true_false((a ^ b ^ r) & Self::half_bit_mask());
        let c = r < a;
        let v = Self::get_overflow(a,b,r);
        let n = Self::get_n(r);
        let z = r == Self::zero();

        f.set(Flags::V, v);
        f.set(Flags::C, c);
        f.set(Flags::H, h);

        r
    }

    fn asl(f : &mut Flags, a : Self) -> Self {
        let c = Self::get_n(a);
        let r = a.wrapping_add(&a);
        let v = c & Self::get_n(r);

        f.set(Flags::C, c);
        f.set(Flags::V, v);
        r
    } 

}

impl GazAlu for u8 {
    fn hi_bit_mask() -> u8 { 0x80u8 }
    fn half_bit_mask() -> u8 { 0x10 }
}

impl GazAlu for u16 {
    fn hi_bit_mask() -> u16 { 0x8000u16 }
    fn half_bit_mask() -> u16 { 0x0000u16 }
}

// {{{ Todo next!
impl  Cpu {

    // Does the H flag as well

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
        self.regs.load_a(v);
    }

    #[inline(always)]
    fn ldu<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let v =  A::fetch_word(mem, &mut self.regs, ins);
        self.regs.load_u(v);
    }

    #[inline(always)]
    fn adda<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.regs.clear_c();
        self.adca::<M,A>(mem,ins);
    }

    #[inline(always)]
    fn adca<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {

        let a = self.regs.a;

        let b = u8::fetch::<A>(mem, &mut self.regs, ins);
        let r = u8::adc(&mut self.regs.flags, a, b);

        self.regs.load_a(r);
    }

    fn anda<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let a = self.regs.a;
        let b = u8::fetch::<A>(mem, &mut self.regs, ins);
        self.regs.load_a( a & b );
    }

    fn andb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let a = self.regs.b;
        let b = u8::fetch::<A>(mem, &mut self.regs, ins);
        self.regs.load_b(a & b);
    }

    #[inline(always)]
    fn adcb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let a = self.regs.b;

        let b = u8::fetch::<A>(mem, &mut self.regs, ins);
        let r = u8::adc(&mut self.regs.flags, a, b);

        self.regs.load_b(r);
    }

    fn addb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.regs.clear_c();
        self.adcb::<M,A>(mem,ins);
    }

    fn lsla_asla<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let a = self.regs.a;
        let r = u8::asl(&mut self.regs.flags, a);
        self.regs.load_a(r)
    }

    fn lslb_aslb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let a = self.regs.b;
        let r = u8::asl(&mut self.regs.flags, a);
        self.regs.load_b(r)
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

    #[inline(always)]
    fn brn<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let offset = A::fetch_byte_as_i16(mem, &mut self.regs, ins);

        if self.regs.flags.contains(Flags::N) {
            ins.next_addr = ins.next_addr.wrapping_add(offset as u16);
        }
    }

    fn addd<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        self.regs.clear_c();

        let a = self.regs.get_d();

        let b = u16::fetch::<A>(mem, &mut self.regs, ins);
        let r = u16::adc(&mut self.regs.flags, a, b);

        self.regs.load_d(r);
    }

    fn andcc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let a = self.regs.flags.bits();
        let b = u8::fetch::<A>(mem, &mut self.regs, ins);
        self.regs.flags.set_flags(a & b);
    }

    fn lsl_asl<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let ea = A::ea(mem, &mut self.regs, ins );

        let v = mem.load_byte(ea);

        let r = u8::asl(&mut self.regs.flags, v);

        self.regs.flags.test_8(r);

        mem.store_byte(ea,r)
    }
}
// }}}

// {{{ Op Codes
impl  Cpu {

    fn asr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("asr NO!")
    }
    fn asra<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("asra NO!")
    }
    fn asrb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("asrb NO!")
    }
    fn beq<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("beq NO!")
    }
    fn bge<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bge NO!")
    }
    fn bgt<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bgt NO!")
    }
    fn bhi<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bhi NO!")
    }
    fn bhs_bcc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bhs_bcc NO!")
    }
    fn bita<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bita NO!")
    }
    fn bitb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bitb NO!")
    }
    fn ble<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("ble NO!")
    }
    fn blo_bcs<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("blo_bcs NO!")
    }
    fn bls<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bls NO!")
    }
    fn blt<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bmi NO!")
    }
    fn bmi<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bmi NO!")
    }
    fn bne<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bne NO!")
    }
    fn bpl<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bpl NO!")
    }
    fn bra<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bra NO!")
    }
    fn bsr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bsr NO!")
    }
    fn bvc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bvc NO!")
    }
    fn bvs<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("bvs NO!")
    }
    fn clr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("clr NO!")
    }
    fn clra<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("clra NO!")
    }
    fn clrb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("clrb NO!")
    }
    fn cmpa<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("cmpa NO!")
    }
    fn cmpb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("cmpb NO!")
    }
    fn cmpx<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("com NO!")
    }
    fn com<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("com NO!")
    }
    fn coma<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("coma NO!")
    }
    fn comb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("comb NO!")
    }
    fn cwai<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("cwai NO!")
    }
    fn daa<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("daa NO!")
    }
    fn dec<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("dec NO!")
    }
    fn deca<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("deca NO!")
    }
    fn decb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("decb NO!")
    }
    fn eora<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("eora NO!")
    }
    fn eorb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("eorb NO!")
    }
    fn exg<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("EXG")
    }
    fn inc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("inc NO!")
    }
    fn inca<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("noy fonr")
    }
    fn incb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("incb NO!")
    }
    fn jmp<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("jmp NO!")
    }
    fn jsr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("jsr NO!")
    }
    fn lbra<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbra NO!")
    }
    fn lbsr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbsr NO!")
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
    fn rts<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("rts NO!")
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
    fn cmpu<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("cmpu NO!")
    }
    fn cmps<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("cmps NO!")
    }
    fn lbrn<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbrn NO!")
    }
    fn lbhi<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbhi NO!")
    }
    fn lbls<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbls NO!")
    }
    fn lbhs_lbcc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbhs_lbcc NO!")
    }
    fn lblo_lbcs<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lblo_lbcs NO!")
    }
    fn lbne<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbne NO!")
    }
    fn lbeq<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbeq NO!")
    }
    fn lbvc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbvc NO!")
    }
    fn lbvs<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbvs NO!")
    }
    fn lbpl<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbpl NO!")
    }
    fn lbmi<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbmi NO!")
    }
    fn lbge<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbge NO!")
    }
    fn lblt<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lblt NO!")
    }
    fn lbgt<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lbgt NO!")
    }
    fn swi2<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("swi2 NO!")
    }
    fn cmpd<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("cmpd NO!")
    }
    fn cmpy<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("cmpy NO!")
    }
    fn ldy<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("ldy NO!")
    }
    fn lble<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lble NO!")
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

