use cpu::mem::MemoryIO;
// use registers::{ Regs};

trait SymTab {
    fn get_symbol(&self, val : u8) -> String;
}

struct Disassembly {
}

#[derive(Default)]
struct Instruction {
    addr : u16,
    next_addr : u16,
    bytes : usize,
    text : String,
    cycles : usize,
}

impl Instruction {
    pub fn new(addr : u16) -> Self {
        Instruction {
            addr : addr,
            next_addr : addr,
            bytes : 0,
            text : String::from(""),
            cycles : 0,
        }
    }
    
    pub fn advance(&mut self, amount : usize) {
        self.next_addr = self.next_addr.wrapping_add(amount as u16);
        self.bytes = self.next_addr.wrapping_sub(self.addr) as usize;
    }

    pub fn next(&mut self) {
        let addr = self.next_addr;
        *self = Self::new(addr)
    }

    fn set_text(&mut self, txt : &String) {
        self.text = txt.clone();
    }

    fn fetch_byte<M : MemoryIO> (&mut self, mem : &mut M) -> u8 {
        let v = mem.load_byte(self.next_addr);
        self.advance(1);
        v
    }

    fn fetch_word<M : MemoryIO> (&mut self, mem : &mut M) -> u16 {
        let v = mem.load_word(self.next_addr);
        self.advance(2);
        v
    }

    fn add_op(&mut self, txt : &'static str) {
        let text = format!("{} {}", txt, self.text);
        self.set_text(&text);
    }

    fn fetch_instruction<M : MemoryIO>(&mut self, mem : &mut M ) -> u16 {
        let a = self.fetch_byte(mem) as u16;

        match a {
            0x10 | 0x11 => (a << 8) + self.fetch_byte(mem) as u16,
            _ => a
        }
    }
}

pub struct Disassembler<M: MemoryIO> {
    mem : M,
    ins : Instruction,
}

impl <M: MemoryIO> Disassembler<M> {

    fn add_op(&mut self, txt : &'static str) -> Disassembly {
        self.ins.add_op(txt);
        Disassembly {}
    }

}

impl <M: MemoryIO> Disassembler<M> {

    fn from_byte_op(&mut self, text : &'static str) -> Disassembly { 
        let v = self.ins.fetch_byte(&mut self.mem);
        let vstr = format!("0x{:02X}", v);
        let op_str = String::from(text);
        self.ins.set_text( &op_str.replace("OP", &vstr));
        Disassembly{}
    }

    fn from_word_op(&mut self, text : &'static str) -> Disassembly { 
        let v = self.ins.fetch_word(&mut self.mem);
        let vstr = format!("0x{:04X}", v);
        let op_str = String::from(text);
        self.ins.set_text( &op_str.replace("OP", &vstr));
        Disassembly{}
    }

    fn from_no_op(&mut self ) -> Disassembly {
        self.ins.set_text(&"".to_string());
        Disassembly {}
    }
}

impl <M: MemoryIO> Disassembler<M> {

    fn direct(&mut self) -> Disassembly { self.from_byte_op("<OP") }

    fn extended(&mut self) -> Disassembly { self.from_word_op("OP") }

    fn immediate8(&mut self) -> Disassembly { self.from_byte_op("#OP") }

    fn immediate16(&mut self) -> Disassembly { self.from_word_op("#OP") }

    fn inherent(&mut self) -> Disassembly { self.from_no_op() }

    fn indexed(&mut self) -> Disassembly {
        panic!("INDEXED NOT IMPLEMENTED")
    }

    fn relative8(&mut self) -> Disassembly {
        let v = self.ins.fetch_byte(&mut self.mem) as i8;
        let vstr = format!("{}", v);
        self.ins.set_text(&vstr);
        Disassembly{}
    }

    fn relative16(&mut self) -> Disassembly {
        let v = self.ins.fetch_word(&mut self.mem) as i16;
        let vstr = format!("{}", v);
        self.ins.set_text(&vstr);
        Disassembly{}
    }
}

impl <M: MemoryIO> Disassembler<M> {

    fn abx(&mut self, diss : Disassembly) -> Disassembly {
        self.add_op("ABX")
    }
    fn adca(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("adca")
    }
    fn adcb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("ADCB")
    }
    fn adda(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("adda")
    }
    fn addb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("addb")
    }
    fn addd(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("addd")
    }
    fn anda(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("anda")
    }
    fn andb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("andb")
    }
    fn andcc(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("andcc")
    }
    fn asr(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("asr")
    }
    fn asra(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("asra")
    }
    fn asrb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("asrb")
    }
    fn beq(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("beq")
    }
    fn bge(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bge")
    }
    fn bgt(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bgt")
    }
    fn bhi(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bhi")
    }
    fn bhs_bcc(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bhs_bcc")
    }
    fn bita(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bita")
    }
    fn bitb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bitb")
    }
    fn ble(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("ble")
    }
    fn blo_bcs(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("blo_bcs")
    }
    fn bls(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bls")
    }
    fn blt(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("blt")
    }
    fn bmi(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bmi")
    }
    fn bne(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bne")
    }
    fn bpl(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bpl")
    }
    fn bra(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bra")
    }
    fn brn(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("brn")
    }
    fn bsr(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bsr")
    }
    fn bvc(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bvc")
    }
    fn bvs(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("bvs")
    }
    fn clr(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("clr")
    }
    fn clra(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("clra")
    }
    fn clrb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("clrb")
    }
    fn cmpa(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("cmpa")
    }
    fn cmpb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("cmpb")
    }
    fn cmpx(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("cmpx")
    }
    fn com(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("com")
    }
    fn coma(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("coma")
    }
    fn comb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("comb")
    }
    fn cwai(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("cwai")
    }
    fn daa(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("daa")
    }
    fn dec(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("dec")
    }
    fn deca(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("deca")
    }
    fn decb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("decb")
    }
    fn eora(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("eora")
    }
    fn eorb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("eorb")
    }
    fn exg(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("exg")
    }
    fn inc(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("inc")
    }
    fn inca(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("inca")
    }
    fn incb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("incb")
    }
    fn jmp(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("jmp")
    }
    fn jsr(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("jsr")
    }
    fn lbra(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbra")
    }
    fn lbsr(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbsr")
    }
    fn lda(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lda")
    }
    fn ldb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("ldb")
    }
    fn ldd(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("ldd")
    }
    fn ldu(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("ldu")
    }
    fn ldx(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("ldx")
    }
    fn leas(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("leas")
    }
    fn leau(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("leau")
    }
    fn leax(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("leax")
    }
    fn leay(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("leay")
    }
    fn lsl_asl(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lsl_asl")
    }
    fn lsla_asla(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lsla_asla")
    }
    fn lslb_aslb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lslb_aslb")
    }
    fn lsr(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lsr")
    }
    fn lsra(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lsra")
    }
    fn lsrb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lsrb")
    }
    fn mul(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("MUL")
    }
    fn neg(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("NEG")
    }
    fn nega(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("nega")
    }
    fn negb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("negb")
    }
    fn nop(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("nop")
    }
    fn ora(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("ora")
    }
    fn orb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("orb")
    }
    fn orcc(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("orcc")
    }
    fn pshs(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("pshs")
    }
    fn pshu(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("pshu")
    }
    fn puls(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("puls")
    }
    fn pulu(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("pulu")
    }
    fn reset(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("reset")
    }
    fn rol(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("rol")
    }
    fn rola(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("rola")
    }
    fn rolb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("rolb")
    }
    fn ror(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("ror")
    }
    fn rora(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("rora")
    }
    fn rorb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("rorb")
    }
    fn rti(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("rti")
    }
    fn rts(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("rts")
    }
    fn sbca(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("sbca")
    }
    fn sbcb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("sbcb")
    }
    fn sex(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("sex")
    }
    fn sta(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("sta")
    }
    fn stb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("stb")
    }
    fn std(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("std")
    }
    fn stu(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("stu")
    }
    fn stx(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("stx")
    }
    fn suba(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("suba")
    }
    fn subb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("subb")
    }
    fn subd(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("subd")
    }
    fn swi(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("swi")
    }
    fn sync(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("sync")
    }
    fn tfr(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("tfr")
    }
    fn tst(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("tst")
    }
    fn tsta(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("tsta")
    }
    fn tstb(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("tstb")
    }
    fn swi3(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("swi3")
    }
    fn cmpu(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("cmpu")
    }
    fn cmps(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("cmps")
    }
    fn lbrn(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbrn")
    }
    fn lbhi(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbhi")
    }
    fn lbls(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbls")
    }
    fn lbhs_lbcc(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbhs_lbcc")
    }
    fn lblo_lbcs(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lblo_lbcs")
    }
    fn lbne(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbne")
    }
    fn lbeq(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbeq")
    }
    fn lbvc(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbvc")
    }
    fn lbvs(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbvs")
    }
    fn lbpl(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbpl")
    }
    fn lbmi(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbmi")
    }
    fn lbge(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbge")
    }
    fn lblt(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lblt")
    }
    fn lbgt(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lbgt")
    }
    fn swi2(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("swi2")
    }
    fn cmpd(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("cmpd")
    }
    fn cmpy(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("cmpy")
    }
    fn ldy(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("ldy")
    }
    fn lble(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("lble")
    }
    fn sty(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("sty")
    }
    fn lds(&mut self, diss :  Disassembly)  -> Disassembly {
        self.add_op("lds")
    }
    fn sts(&mut self, diss : Disassembly)  -> Disassembly {
        self.add_op("sts")
    }

    fn unimplemented(&mut self) -> Disassembly {
        panic!("??? Unimplemented")
    }

    pub fn new(mem : M) -> Self {
        Disassembler {
            mem : mem,
            ins : Default::default(), }
    }

    pub fn diss(&mut self, addr : u16, amount : usize) {
        self.ins = Instruction::new(addr);

        for i in 0..amount {

            let op = self.ins.fetch_instruction(&mut self.mem);
            let d = decode_op!(op, self);

            let bstr = self.mem.get_mem_as_str(self.ins.addr, self.ins.bytes as u16 );
            println!("0x{:04X}   {:15} {}", self.ins.addr, bstr, self.ins.text);

            self.ins.next();
        }

    }

}
