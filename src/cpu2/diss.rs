use cpu2::Cpu;
use mem::MemoryIO;
// use registers::{ Regs};

struct Disassembler<M: MemoryIO> {
    cpu : Cpu<M>,
}

impl <M: MemoryIO> Disassembler<M> {

    pub fn direct(&mut self) -> String {
        panic!("NO!")
    }

    pub fn extended(&mut self) -> String {
        panic!("NO!")
    }

    pub fn immediate8(&mut self) -> String {
        panic!("NO!")
    }

    pub fn immediate16(&mut self) -> String {
        panic!("NO!")
    }

    pub fn indexed(&mut self) -> String {
        panic!("NO!")
    }
    
    pub fn inherent(&mut self) -> String {
        panic!("NO!")
    }

    pub fn relative(&mut self) -> String {
        panic!("NO!")
    }

    fn abx(&mut self, txt : String) -> String {
        panic!("NO!")
    }
    pub fn adca(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn adcb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn adda(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn addb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn addd(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn anda(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn andb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn andcc(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn asr(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn asra(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn asrb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn beq(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bge(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bgt(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bhi(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bhs_bcc(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bita(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bitb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn ble(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn blo_bcs(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bls(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn blt(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bmi(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bne(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bpl(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bra(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn brn(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bsr(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bvc(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn bvs(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn clr(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn clra(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn clrb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn cmpa(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn cmpb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn cmpx(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn com(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn coma(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn comb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn cwai(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn daa(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn dec(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn deca(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn decb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn eora(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn eorb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn exg(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn inc(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn inca(&mut self, txt : String)  -> String {
        panic!("noy fonr")
    }
    pub fn incb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn jmp(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn jsr(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbra(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbsr(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lda(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn ldb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn ldd(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn ldu(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn ldx(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn leas(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn leau(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn leax(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn leay(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lsl_asl(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lsla_asla(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lslb_aslb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lsr(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lsra(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lsrb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn mul(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn neg(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn nega(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn negb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn nop(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn ora(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn orb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn orcc(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn pshs(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn pshu(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn puls(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn pulu(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn reset(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn rol(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn rola(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn rolb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn ror(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn rora(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn rorb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn rti(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn rts(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn sbca(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn sbcb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn sex(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn sta(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn stb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn std(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn stu(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn stx(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn suba(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn subb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn subd(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn swi(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn sync(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn tfr(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn tst(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn tsta(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn tstb(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn swi3(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn cmpu(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn cmps(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbrn(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbhi(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbls(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbhs_lbcc(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lblo_lbcs(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbne(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbeq(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbvc(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbvs(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbpl(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbmi(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbge(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lblt(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lbgt(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn swi2(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn cmpd(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn cmpy(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn ldy(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lble(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn sty(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn lds(&mut self, txt : String)  -> String {
        panic!("NO!")
    }
    pub fn sts(&mut self, txt : String)  -> String {
        panic!("NO!")
    }

    pub fn unimplemented(&mut self) -> String {
        panic!("???")
    }

    pub fn new(cpu : Cpu<M>) -> Self {
        Disassembler {
            cpu : cpu
        }
    }

    pub fn disassemble(&mut self, addr : u16, amount : usize) {
        self.cpu.regs.pc = addr;
        let op = self.cpu.fetch_instruction();
        decode_op!(op, self);
    }

}
