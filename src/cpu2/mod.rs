use mem::MemoryIO;
use registers::{ Regs};

#[macro_use]
mod isa;
mod diss;


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

struct Cpu<M: MemoryIO> {
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
    fn fetch8(&self, cpu: &mut Cpu<M>) -> u8 { 
        cpu.mem.load_byte(self.0) 
    }

    fn fetch16(&self, cpu: &mut Cpu<M>) -> u16 { 
        cpu.mem.load_word(self.0) 
    }

    fn store8(&self, cpu : &mut Cpu<M>, val : u8) {
        cpu.mem.store_byte(self.0, val)
    }

    fn store16(&self, cpu : &mut Cpu<M>, val : u16) {
        cpu.mem.store_word(self.0, val)
    }
}

impl <M: MemoryIO> AddressingMode<M> for InherentAddressingMode {}
impl <M: MemoryIO> AddressingMode<M> for ExtendedAddressingMode {}


impl <M: MemoryIO> Cpu<M> {

    pub fn direct(&mut self) -> MemoryAddressingMode {
        let v = self.fetch_u8_bump_pc() as u16;
        let addr = ((self.regs.dp as u16) << 8) + v;
        MemoryAddressingMode( addr )
    }

    pub fn extended(&mut self) -> ExtendedAddressingMode {
        panic!("NO!")
    }

    pub fn immediate8(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode( self.add_pc(1) )
    }

    pub fn immediate16(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode( self.add_pc(2) )
    }

    pub fn indexed(&mut self) -> MemoryAddressingMode {
        panic!("NO!")
    }
    
    pub fn inherent(&mut self) -> InherentAddressingMode {
        panic!("NO!")
    }

    pub fn relative(&mut self) -> MemoryAddressingMode {
        panic!("NO!")
    }

}

//}}}

// {{{ Op Codes
impl <M: MemoryIO> Cpu<M> {

    fn abx<A : AddressingMode<M>>(&mut self, addr_mode : A) {
        panic!("NO!")
    }

    pub fn adca<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn adcb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn adda<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn addb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn addd<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn anda<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn andb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn andcc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn asr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn asra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn asrb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn beq<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bge<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bgt<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bhi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bhs_bcc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bita<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bitb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn ble<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn blo_bcs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bls<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn blt<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bmi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bne<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bpl<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn brn<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bsr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bvc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn bvs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn clr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn clra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn clrb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn cmpa<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn cmpb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn cmpx<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn com<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn coma<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn comb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn cwai<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn daa<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn dec<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn deca<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn decb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn eora<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn eorb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn exg<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn inc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn inca<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("noy fonr")
    }
    pub fn incb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn jmp<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn jsr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn lbsr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    // Loads
    pub fn lda<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch8(self);
        self.regs.load_a(operand)
    }

    pub fn ldb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch8(self);
        self.regs.load_b(operand)
    }

    pub fn ldd<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch16(self);
        self.regs.load_d(operand)
    }

    pub fn ldu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch16(self);
        self.regs.load_u(operand)
    }

    pub fn ldx<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        let operand = addr_mode.fetch16(self);
        self.regs.load_x(operand)
    }

    pub fn leas<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn leau<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn leax<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn leay<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn lsl_asl<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn lsla_asla<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn lslb_aslb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn lsr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn lsra<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn lsrb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn mul<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn neg<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn nega<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn negb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn nop<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn ora<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn orb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn orcc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn pshs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn pshu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn puls<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn pulu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn reset<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn rol<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn rola<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn rolb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn ror<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn rora<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn rorb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn rti<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn rts<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn sbca<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn sbcb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn sex<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn sta<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn stb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn std<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn stu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn stx<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn suba<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn subb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn subd<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn swi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn sync<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn tfr<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn tst<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn tsta<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }
    pub fn tstb<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn swi3<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn cmpu<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn cmps<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbrn<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbhi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbls<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbhs_lbcc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lblo_lbcs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbne<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbeq<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbvc<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbvs<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbpl<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbmi<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbge<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lblt<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lbgt<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn swi2<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn cmpd<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn cmpy<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn ldy<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lble<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn sty<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn lds<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn sts<A : AddressingMode<M>>(&mut self, addr_mode : A)  {
        panic!("NO!")
    }

    pub fn unimplemented(&mut self) {
        panic!("unimplemnted op code")
    }

    pub fn fetch_instruction(&mut self) -> u16 {

        let a = self.fetch_u8_bump_pc() as u16;

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

