use cpu::mem::MemoryIO;
use cpu::registers::{ Regs};

// Addressing modes

trait AddressingMode<M : MemoryIO> {

    fn fetch8(&self, cpu: &mut Cpu<M>) -> u8 {
        panic!("Unimplimneted fetch ")
    }

    fn fetch16(&self, cpu: &mut Cpu<M>) -> u16 {
        panic!("Unimplimneted fetch ")
    }

    fn store8(&self, cpu: &mut Cpu<M>, val : u8) {
        panic!("unimplemented")
    }

    fn store16(&self, cpu: &mut Cpu<M>, val : u16) {
        panic!("unimplemented")
    }
}

// {{{

pub struct Cpu<M: MemoryIO> {
    pub mem: M,
    pub regs: Regs,
}

impl <M: MemoryIO> Cpu<M> {

    pub fn add_pc(&mut self, to_add : u16) -> u16 {
        // add 1 to the pc, return the old pc
        let old_pc = self.regs.pc;
        self.regs.pc = old_pc.wrapping_add(to_add);
        old_pc
    }

    pub fn peek_u8_pc(&mut self) -> u8 {
        let pc = self.regs.pc;
        self.mem.load_byte(pc) as u8
    }

    pub fn peek_u16_pc(&mut self) -> u16 {
        let pc = self.regs.pc;
        self.mem.load_word(pc)
    }

    pub fn fetch_u8_bump_pc(&mut self) -> u8 {
        let pc =  self.add_pc(1);
        self.mem.load_byte(pc)
    }

    pub fn fetch_u16_bump_pc(&mut self) -> u16 {
        let pc = self.add_pc(2);
        self.mem.load_word(pc)
    }

    fn step(&mut self) -> &mut Self {
        let op = self.fetch_u8_bump_pc();
        self
    }

    pub fn new(mem : M) -> Cpu<M> {
        Cpu {
            mem: mem,
            regs: Regs::new(),
        }
    }
}

//{{{ Addressing modes

struct ExtendedAddressingMode;
struct MemoryAddressingMode(u16);
struct IndexedAddressingMode;
struct InherentAddressingMode;
struct RelativeAddressingMode;

impl <M: MemoryIO> AddressingMode<M> for MemoryAddressingMode { 
    fn fetch8(&self, cpu: &mut Cpu<M>) -> u8 { cpu.mem.load_byte(self.0) }
    fn fetch16(&self, cpu: &mut Cpu<M>) -> u16 { cpu.mem.load_word(self.0) }
    fn store8(&self, cpu : &mut Cpu<M>, val : u8) { cpu.mem.store_byte(self.0, val) }
    fn store16(&self, cpu : &mut Cpu<M>, val : u16) { cpu.mem.store_word(self.0, val) }
}

impl <M: MemoryIO> AddressingMode<M> for InherentAddressingMode {}
impl <M: MemoryIO> AddressingMode<M> for ExtendedAddressingMode {}

impl <M: MemoryIO> Cpu<M> {

    fn direct(&mut self) -> MemoryAddressingMode {
        let v = self.fetch_u8_bump_pc() as u16;
        let addr = ((self.regs.dp as u16) << 8) + v;
        MemoryAddressingMode( addr )
    }

    fn extended(&mut self) -> ExtendedAddressingMode {
        panic!("NO!")
    }

    fn immediate8(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode( self.add_pc(1) )
    }

    fn immediate16(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode( self.add_pc(2) )
    }

    fn indexed(&mut self) -> MemoryAddressingMode {
        panic!("NO!")
    }

    fn inherent(&mut self) -> InherentAddressingMode {
        panic!("NO!")
    }

    fn relative8(&mut self) -> MemoryAddressingMode {
        panic!("NO!")
    }
    fn relative16(&mut self) -> MemoryAddressingMode {
        panic!("NO!")
    }


}

//}}}

// {{{ Op Codes
impl <M: MemoryIO> Cpu<M> {

    fn abx<A : AddressingMode<M>>(&mut self, addr_mode : A) {
        panic!("NO!")
    }

    fn adca<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn adcb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn adda<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn addb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn addd<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn anda<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn andb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn andcc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn asr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn asra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn asrb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn beq<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bge<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bgt<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bhi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bhs_bcc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bita<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bitb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn ble<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn blo_bcs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bls<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn blt<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bmi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bne<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bpl<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn brn<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bsr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bvc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn bvs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn clr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn clra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn clrb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn cmpa<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn cmpb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn cmpx<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn com<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn coma<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn comb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn cwai<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn daa<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn dec<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn deca<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn decb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn eora<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn eorb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn exg<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn inc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn inca<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("noy fonr")
    }
    fn incb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn jmp<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn jsr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn lbsr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    // Loads
    fn lda<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch8(self);
        self.regs.load_a(operand)
    }

    fn ldb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch8(self);
        self.regs.load_b(operand)
    }

    fn ldd<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch16(self);
        self.regs.load_d(operand)
    }

    fn ldu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch16(self);
        self.regs.load_u(operand)
    }

    fn ldx<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch16(self);
        self.regs.load_x(operand)
    }

    fn leas<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn leau<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn leax<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn leay<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn lsl_asl<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn lsla_asla<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn lslb_aslb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn lsr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn lsra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn lsrb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn mul<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn neg<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn nega<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn negb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn nop<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn ora<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn orb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn orcc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn pshs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn pshu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn puls<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn pulu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn reset<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn rol<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn rola<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn rolb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn ror<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn rora<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn rorb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn rti<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn rts<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn sbca<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn sbcb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn sex<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn sta<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn stb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn std<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn stu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn stx<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn suba<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn subb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn subd<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn swi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn sync<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn tfr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn tst<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn tsta<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    fn tstb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn swi3<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn cmpu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn cmps<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbrn<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbhi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbls<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbhs_lbcc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lblo_lbcs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbne<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbeq<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbvc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbvs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbpl<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbmi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbge<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lblt<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lbgt<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn swi2<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn cmpd<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn cmpy<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn ldy<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lble<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn sty<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn lds<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn sts<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    fn unimplemented(&mut self) {
        panic!("unimplemnted op code")
    }

    pub fn fetch_instruction(&mut self) -> u16 {

        let a = self.fetch_u8_bump_pc() as u16;

        println!("fetch ins 0x{:04x}", a);

        match a {
            0x10 | 0x11 => (a << 8) + self.fetch_u8_bump_pc() as u16,
            _ => a
        }
    }

    pub fn exec(&mut self, num : usize) {
        let a = self.fetch_instruction();
        decode_op!(a, self)
    }
}

//
// }}}

