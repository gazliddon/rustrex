use mem::MemoryIO;
// use cpu::registers::{ Regs, RegEnum};
use cpu::{ Regs};

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
    }

    fn inherent_reg_stack<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) { 
        panic!("not yet")
    }

    fn inherent_reg_reg<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) { 
        panic!("not yet")
    }

    fn indexed<M: MemoryIO>(&mut self, mem : &M, ins : &mut InstructionDecoder) {
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
    fn tfr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lds<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn abx<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder) {
        panic!("NO!")
    }
    fn adca<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
}
// }}}

// {{{ Op Codes
impl  Cpu {
    fn adcb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn adda<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn addb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn addd<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn anda<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn andb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn andcc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn asr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn asra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn asrb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn beq<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bge<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bgt<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bhi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bhs_bcc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bita<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bitb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn ble<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn blo_bcs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bls<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn blt<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bmi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bne<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bpl<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn brn<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bsr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bvc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn bvs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn clr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn clra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn clrb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn cmpa<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn cmpb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn cmpx<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn com<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn coma<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn comb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn cwai<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn daa<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn dec<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn deca<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn decb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn eora<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn eorb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn exg<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn inc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn inca<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("noy fonr")
    }
    fn incb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn jmp<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn jsr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbsr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn ldb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn ldd<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn leas<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn leau<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn leax<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn leay<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lsl_asl<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lsla_asla<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lslb_aslb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lsr<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lsra<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lsrb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn mul<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn neg<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn nega<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn negb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn nop<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn ora<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn orb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn pshs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn pshu<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn puls<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn pulu<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn reset<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn rol<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn rola<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn rolb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn ror<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn rora<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn rorb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn rti<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn rts<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn sbca<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn sbcb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn sex<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn stb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn std<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn stu<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn suba<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn subb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn subd<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn swi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn sync<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn tst<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn tsta<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn tstb<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn swi3<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn cmpu<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn cmps<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbrn<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbhi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbls<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbhs_lbcc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lblo_lbcs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbne<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbeq<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbvc<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbvs<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbpl<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbmi<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbge<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lblt<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lbgt<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn swi2<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn cmpd<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn cmpy<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn ldy<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn lble<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn sty<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
    }
    fn sts<M: MemoryIO>(&mut self, mem : &mut M, ins : &InstructionDecoder)  {
        panic!("NO!")
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

