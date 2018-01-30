use mem::MemoryIO;
use cpu::{ Regs, RegEnum, Flags, InstructionDecoder};

use cpu::{AddressLines, Direct, Extended, Immediate, Inherent, Relative, Indexed};

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

pub fn test1<A: AddressLines>() -> u8 {
    0
}

pub fn test2() {
    let a = test1::<Direct>();

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
//{{{ Helpers
impl Cpu {
    fn adc_helper(&mut self, i0 : u8, i1 : u8) -> u8 {
        let c  = self.regs.get_c();

        let r = ( i0.wrapping_add(i1) ).wrapping_add(c);

        let mut f = self.regs.flags;

        f.set(Flags::H, false);
        f.set(Flags::V, Flags::get_v(i0, i1, r));
        f.set(Flags::C, false);

        r
    }
}
// }}}


// {{{ Todo next!
impl  Cpu {
    #[inline(always)]
    fn orcc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let v = A::fetch_byte(mem, &mut self.regs, ins);
        self.regs.flags.or_flags(v);
    }

    #[inline(always)]
    fn ldx<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let v =  A::fetch_word(mem, &mut self.regs, ins);
        self.regs.load_x(v);
    }

    #[inline(always)]
    fn stx<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let addr = A::fetch_word(mem, &mut self.regs, ins);
        mem.store_word(addr, self.regs.x);
    }

    #[inline(always)]
    fn sta<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let addr = A::fetch_word(mem, &mut self.regs, ins);
        mem.store_byte(addr, self.regs.a);
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
        let operand = A::fetch_byte(mem, &mut self.regs, ins);
        let v  = self.regs.a;
        let r = self.adc_helper(v, operand);
        self.regs.load_a(r);
    }

    #[inline(always)]
    fn tfr<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let operand = A::fetch_byte(mem, &mut self.regs, ins);
        let (a,b) = get_tfr_regs(operand as u8);
        let av = self.regs.get(a);
        self.regs.set(b, av);
    }

    #[inline(always)]
    fn lds<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let v =  A::fetch_word(mem, &mut self.regs, ins);
        self.regs.load_s(v);
    }

    #[inline(always)]
    fn abx<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        let x = self.regs.x;
        self.regs.x = x.wrapping_add(self.regs.b as u16);
    }
}
// }}}

// {{{ Op Codes
impl  Cpu {

    fn adcb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("adcb NO!")
    }

    fn addb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("addb NO!")
    }
    fn addd<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("addd NO!")
    }
    fn anda<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("anda NO!")
    }
    fn andb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("andb NO!")
    }
    fn andcc<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("andcc NO!")
    }
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
    fn brn<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("brn NO!")
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
    fn lsl_asl<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lsl_asl NO!")
    }
    fn lsla_asla<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lsla_asla NO!")
    }
    fn lslb_aslb<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        panic!("lslb_aslb NO!")
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

    fn sts<M: MemoryIO, A : AddressLines>(&mut self, mem : &mut M, ins : &mut InstructionDecoder)  {
        mem.store_word(ins.operand, self.regs.s);
        self.regs.flags.test_16(self.regs.s)
    }

    fn unimplemented(&mut self, ins : &mut InstructionDecoder) {
        panic!("unimplemnted op code")
    }

    fn get_pc(&self) -> u16 {
        self.regs.pc
    }

    pub fn step<M: MemoryIO>(&mut self, mem : &mut M) -> InstructionDecoder {

        let mut ins = InstructionDecoder::new(self.regs.pc);

        let op = ins.fetch_instruction(mem);

        macro_rules! handle_op {
            ($addr:ident, $action:ident) => (
                {
                    self.$action::<M, $addr>(mem, &mut ins);
                }
            )
        }

        op_table!(op, 
                  {self.unimplemented(&mut ins)}
                  );

        self.regs.pc = ins.next_addr;

        ins
    }
}

//
// }}}

