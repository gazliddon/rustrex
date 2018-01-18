use mem::MemoryIO;
use cpu::{ Regs, RegEnum, Flags };

#[derive(Default)]
#[derive(Debug)]
pub struct InstructionDecoder {
    pub op_code : u16,
    pub cycles : usize,
    pub addr : u16,
    pub bytes : usize,
    pub next_addr : u16,
    pub mem : [u8; 4],
    pub operand : u16,
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

pub fn get_tfr_regs(op : u8) -> (RegEnum, RegEnum) {
    ( get_tfr_reg(op>>4), get_tfr_reg(op&0xf) )
}

impl InstructionDecoder {

    pub fn new(addr: u16)-> Self {
        InstructionDecoder {
            addr : addr,
            next_addr : addr,
            .. Default::default()
        }
    }

    pub fn inc_cycles(&mut self) -> usize {
        self.cycles = self.cycles + 1;
        self.cycles
    }

    pub fn fetch_byte<M : MemoryIO>(&mut self, mem: &M) -> u8 {
        let b = mem.load_byte(self.next_addr);
        self.next_addr = self.next_addr.wrapping_add(1);

        self.mem[self.bytes] = b;
        self.bytes = self.bytes + 1;

        b
    }

    pub fn fetch_word<M : MemoryIO>(&mut self, mem: &M) -> u16 {
        let w = mem.load_word(self.next_addr);
        self.next_addr = self.next_addr.wrapping_add(2);

        self.mem[self.bytes] = ((w >> 8) & 0xff) as u8;
        self.mem[self.bytes+1] = w as u8;
        self.bytes = self.bytes + 2;
        w   
    }

    pub fn get_next_addr(&self) -> u16 {
        self.next_addr
    }

    pub fn fetch_instruction<M: MemoryIO>(&mut self, mem: &M) -> u16 {

        let a = self.fetch_byte(mem) as u16;

        self.op_code = match a {
            0x10 | 0x11 => {
                (a << 8) + self.fetch_byte(mem) as u16
            }
            _ => a
        };

        self.op_code
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

//{{{ Addressing modes

impl Cpu {


    fn direct<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) { 
        let index = ins.fetch_byte(mem) as u16;
        ins.operand = self.regs.get_dp_ptr().wrapping_add(index);
    }

    fn extended<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) { 
        ins.operand = ins.fetch_word(mem);
    }

    fn immediate8<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) { 
        ins.operand = ins.fetch_byte(mem) as u16;
    }

    fn immediate16<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) { 
        ins.operand = ins.fetch_word(mem);
    }

    fn inherent<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) {
        //don't do anything with inherent
    }

    fn inherent_reg_stack<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) { 
        panic!("no inherent reg stack")
    }

    fn inherent_reg_reg<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) { 
        ins.operand = ins.fetch_byte(mem) as u16;
    }

    fn indexed<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) {
        panic!("no indexed")
    }

    fn relative8<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) {
        let offset = ins.fetch_byte(mem) as i8;
    }

    fn relative16<M: MemoryIO>(&mut self, mem: &M, ins : &mut InstructionDecoder) {
        let offset = ins.fetch_word(mem) as i16;
    }


}

//}}}

// {{{ Todo next!
impl  Cpu {
    fn orcc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        self.regs.flags.assign_flags(ins.operand as u8);
    }

    fn ldx<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        self.regs.load_x(ins.operand);
    }

    fn stx<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        mem.store_word(ins.operand, self.regs.x);
    }
    fn sta<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        mem.store_byte(ins.operand, self.regs.a);
    }

    fn lda<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        self.regs.load_a(ins.operand as u8);
    }
    fn ldu<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        self.regs.load_u(ins.operand);
    }

    fn adda<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        self.regs.clear_c();
        let v  = self.regs.a;
        let r = self.adc_helper(v, ins.operand as u8);
        self.regs.load_a(r);
    }

    fn adca<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        let v  = self.regs.a;
        let r = self.adc_helper(v, ins.operand as u8);
        self.regs.load_a(r);
    }

    fn tfr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        let (a,b) = get_tfr_regs(ins.operand as u8);
        let av = self.regs.get(a);
        self.regs.set(b, av);
    }

    fn lds<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        self.regs.load_u( mem.load_word(ins.operand) );
    }

    fn abx<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder) {
        let x = self.regs.x;
        self.regs.x = x.wrapping_add(self.regs.b as u16);
    }

}
// }}}

// {{{ Op Codes
impl  Cpu {
    fn adcb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("adcb NO!")
    }
    fn addb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("addb NO!")
    }
    fn addd<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("addd NO!")
    }
    fn anda<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("anda NO!")
    }
    fn andb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("andb NO!")
    }
    fn andcc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("andcc NO!")
    }
    fn asr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("asr NO!")
    }
    fn asra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("asra NO!")
    }
    fn asrb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("asrb NO!")
    }
    fn beq<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("beq NO!")
    }
    fn bge<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bge NO!")
    }
    fn bgt<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bgt NO!")
    }
    fn bhi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bhi NO!")
    }
    fn bhs_bcc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bhs_bcc NO!")
    }
    fn bita<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bita NO!")
    }
    fn bitb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bitb NO!")
    }
    fn ble<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("ble NO!")
    }
    fn blo_bcs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("blo_bcs NO!")
    }
    fn bls<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bls NO!")
    }
    fn blt<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bmi NO!")
    }
    fn bmi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bmi NO!")
    }
    fn bne<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bne NO!")
    }
    fn bpl<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bpl NO!")
    }
    fn bra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bra NO!")
    }
    fn brn<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("brn NO!")
    }
    fn bsr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bsr NO!")
    }
    fn bvc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bvc NO!")
    }
    fn bvs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("bvs NO!")
    }
    fn clr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("clr NO!")
    }
    fn clra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("clra NO!")
    }
    fn clrb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("clrb NO!")
    }
    fn cmpa<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("cmpa NO!")
    }
    fn cmpb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("cmpb NO!")
    }
    fn cmpx<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("com NO!")
    }
    fn com<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("com NO!")
    }
    fn coma<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("coma NO!")
    }
    fn comb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("comb NO!")
    }
    fn cwai<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("cwai NO!")
    }
    fn daa<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("daa NO!")
    }
    fn dec<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("dec NO!")
    }
    fn deca<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("deca NO!")
    }
    fn decb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("decb NO!")
    }
    fn eora<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("eora NO!")
    }
    fn eorb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("eorb NO!")
    }

    fn exg<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("EXG")
    }

    fn inc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("inc NO!")
    }
    fn inca<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("noy fonr")
    }
    fn incb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("incb NO!")
    }
    fn jmp<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("jmp NO!")
    }
    fn jsr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("jsr NO!")
    }
    fn lbra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbra NO!")
    }
    fn lbsr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbsr NO!")
    }
    fn ldb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("ldb NO!")
    }
    fn ldd<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("ldd NO!")
    }
    fn leas<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("leas NO!")
    }
    fn leau<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("leau NO!")
    }
    fn leax<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("leax NO!")
    }
    fn leay<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("leay NO!")
    }
    fn lsl_asl<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lsl_asl NO!")
    }
    fn lsla_asla<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lsla_asla NO!")
    }
    fn lslb_aslb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lslb_aslb NO!")
    }
    fn lsr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lsr NO!")
    }
    fn lsra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lsra NO!")
    }
    fn lsrb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lsrb NO!")
    }
    fn mul<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("mul NO!")
    }
    fn neg<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("neg NO!")
    }
    fn nega<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("nega NO!")
    }
    fn negb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("negb NO!")
    }
    fn nop<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("nop NO!")
    }
    fn ora<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("ora NO!")
    }
    fn orb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("orb NO!")
    }
    fn pshs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("pshs NO!")
    }
    fn pshu<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("pshu NO!")
    }
    fn puls<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("puls NO!")
    }
    fn pulu<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("pulu NO!")
    }
    fn reset<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("reset NO!")
    }
    fn rol<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("rol NO!")
    }
    fn rola<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("rola NO!")
    }
    fn rolb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("rolb NO!")
    }
    fn ror<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("ror NO!")
    }
    fn rora<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("rora NO!")
    }
    fn rorb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("rorb NO!")
    }
    fn rti<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("rti NO!")
    }
    fn rts<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("rts NO!")
    }
    fn sbca<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("sbca NO!")
    }
    fn sbcb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("sbcb NO!")
    }
    fn sex<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("sex NO!")
    }
    fn stb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("stb NO!")
    }
    fn std<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("std NO!")
    }
    fn stu<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("stu NO!")
    }
    fn suba<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("suba NO!")
    }
    fn subb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("subb NO!")
    }
    fn subd<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("subd NO!")
    }
    fn swi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("swi NO!")
    }
    fn sync<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("sync NO!")
    }
    fn tst<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("tst NO!")
    }
    fn tsta<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("tsta NO!")
    }
    fn tstb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("tstb NO!")
    }
    fn swi3<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("swi3 NO!")
    }
    fn cmpu<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("cmpu NO!")
    }
    fn cmps<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("cmps NO!")
    }
    fn lbrn<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbrn NO!")
    }
    fn lbhi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbhi NO!")
    }
    fn lbls<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbls NO!")
    }
    fn lbhs_lbcc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbhs_lbcc NO!")
    }
    fn lblo_lbcs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lblo_lbcs NO!")
    }
    fn lbne<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbne NO!")
    }
    fn lbeq<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbeq NO!")
    }
    fn lbvc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbvc NO!")
    }
    fn lbvs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbvs NO!")
    }
    fn lbpl<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbpl NO!")
    }
    fn lbmi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbmi NO!")
    }
    fn lbge<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbge NO!")
    }
    fn lblt<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lblt NO!")
    }
    fn lbgt<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lbgt NO!")
    }
    fn swi2<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("swi2 NO!")
    }
    fn cmpd<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("cmpd NO!")
    }
    fn cmpy<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("cmpy NO!")
    }
    fn ldy<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("ldy NO!")
    }
    fn lble<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("lble NO!")
    }
    fn sty<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("sty NO!")
    }

    fn sts<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        mem.store_word(ins.operand, self.regs.s);
        self.regs.flags.test_16(self.regs.s)
    }

    fn unimplemented(&mut self, op_code: u16) {
        panic!("unimplemnted op code")
    }

    fn get_pc(&self) -> u16 {
        self.regs.pc
    }

    pub fn step<M: MemoryIO>(&mut self, mem : &mut M) -> InstructionDecoder {

        let mut ins = InstructionDecoder::new(self.regs.pc);

        let op = ins.fetch_instruction(mem);

        decode_op!(op, self, mem, &mut ins);

        self.regs.pc = ins.next_addr;

        ins
    }
}

//
// }}}

