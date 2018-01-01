// use std::vec::Vec;

use cpu::Cpu;
use memmap::MemMap;
use mem::MemoryIO;
use std::fmt;

use addr::AddrModes;
use addr::fetch_operand;


pub enum FlagEffects {
    UNAFFECTED,
    AFFECTED,
    RESET,
    SET,
    UNKNOWN
}

pub struct AddrMode {
    pub name : &'static str,
    pub mode : AddrModes,
}

pub struct Op {
    pub mnenomic : &'static str, 
    pub exec : fn( _a : u16, _b : u16, _c : u16, _cpu : &mut Cpu, _mem : &mut MemMap ) -> u16,
}

impl fmt::Debug for Op {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mnenomic)
    }
}

trait CanAddress {
    fn get_name(&self) -> &'static str;
}

impl CanAddress for AddrMode {
    fn get_name(&self) -> &'static str {
        self.name
    }
}

pub struct Ins {
    pub op : &'static Op,
    pub addr_mode : &'static AddrMode,
    pub op_code : u16,
    pub cycles : u8,
    pub bytes : u8,
    pub operand_offset : u8,
}


// const TS : TestStruct  = TestStruct {
//     addr_mode : ILLEGAL_BOX,
// };

impl Ins {

    pub fn exec(&self,  cpu : &mut Cpu, mem : &mut MemMap) -> u32 {

        // let pc = cpu.regs.pc;

        // let (_, operand) =  fetch_operand(&self.addr_mode.mode,
        //                              self.op_code,
        //                              cpu,
        //                              mem,
        //                              pc + 1);
        // let exec =  self.op.exec;

        // let res = exec(operand,0,0,cpu, mem);

        // self.cycles as u32
        0
    }
}

// Addressing modes
static ILLEGAL_ADDR : AddrMode = AddrMode {
    name : "illegal",
    mode : AddrModes::Illegal,
};

static DIRECT : AddrMode = AddrMode {
    name : "DIRECT",
    mode : AddrModes::Direct,
};

static INHERENT : AddrMode = AddrMode {
    name : "INHERENT",
    mode : AddrModes::Inherent,
};

static VARIANT : AddrMode = AddrMode {
    name : "VARIANT",
    mode : AddrModes::Variant,
};

static RELATIVE : AddrMode = AddrMode {
    name : "RELATIVE",
    mode : AddrModes::Relative,
};

static INDEXED : AddrMode = AddrMode {
    name : "INDEXED",
    mode : AddrModes::Indexed,
};

static IMMEDIATE : AddrMode = AddrMode {
    name : "IMMEDIATE",
    mode : AddrModes::Immediate,
};

static EXTENDED : AddrMode = AddrMode {
    name : "EXTENDED",
    mode : AddrModes::Extended,
};



// Stack helpers
fn pushs_byte(v : u8, cpu : &mut Cpu, mem : &mut MemMap ) {
    let s = cpu.regs.s ;
    mem.store_byte(s -1, v) ;
    cpu.regs.s = s - 1;
}

fn pushs_word(_v : u16, cpu : &mut Cpu, mem : &mut MemMap ) {
    pushs_byte(0, cpu,mem);
    pushs_byte(0, cpu,mem);
}

fn pushu_byte(v : u8, cpu : &mut Cpu, mem : &mut MemMap ) {
    let u = cpu.regs.u ;
    mem.store_byte(u -1, v) ;
    cpu.regs.u = u - 1;
}

fn pushu_word(_v : u16, cpu : &mut Cpu, mem : &mut MemMap ) {
    pushu_byte(0, cpu,mem);
    pushu_byte(0, cpu,mem);
}

// Opcodes
fn default_exec ( _a : u16, _b : u16, _c : u16, _cpu : &mut Cpu, _mem : &mut MemMap ) -> u16 {
    0
}

pub fn to8i(_a : u16) -> i8 {
    (_a & 0xff) as i8 
}

pub fn neg_exec( _a : u16, _b : u16, _c : u16, _cpu : &mut Cpu, _mem : &mut MemMap ) -> u16 {
    let _a8 = -to8i(_a);
    _a8 as u16
}

static NEG : Op = Op {
    mnenomic: "NEG",
    exec: neg_exec,
};

static COM : Op = Op {
    mnenomic : "COM",
    exec: default_exec,
};

static LSR : Op = Op {
    mnenomic : "LSR",
    exec: default_exec,
};

static ROR : Op = Op {
    mnenomic : "ROR",
    exec: default_exec,
};

static ASR : Op = Op {
    mnenomic : "ASR",
    exec: default_exec,
};

static ILLEGAL : Op = Op {
    mnenomic : "???",
    exec: default_exec,
};

static ANDCC : Op = Op {
    mnenomic : "ANDCC",
    exec: default_exec,
};

static BEQ : Op = Op {
    mnenomic : "BEQ",
    exec: default_exec,
};

static BGE : Op = Op {
    mnenomic : "BGE",
    exec: default_exec,
};

static BHI : Op = Op {
    mnenomic : "BHI",
    exec: default_exec,
};

static BHS_BCC : Op = Op {
    mnenomic : "BHS_BCC",
    exec: default_exec,
};

static BLO_BCS : Op = Op {
    mnenomic : "BLO_BCS",
    exec: default_exec,
};

static BLS : Op = Op {
    mnenomic : "BLS",
    exec: default_exec,
};

static BMI : Op = Op {
    mnenomic : "BMI",
    exec: default_exec,
};

static BNE : Op = Op {
    mnenomic : "BNE",
    exec: default_exec,
};

static BPL : Op = Op {
    mnenomic : "BPL",
    exec: default_exec,
};

static BRA : Op = Op {
    mnenomic : "BRA",
    exec: default_exec,
};

static BRN : Op = Op {
    mnenomic : "BRN",
    exec: default_exec,
};

static BVC : Op = Op {
    mnenomic : "BVC",
    exec: default_exec,
};

static BVS : Op = Op {
    mnenomic : "BVS",
    exec: default_exec,
};

static CLR : Op = Op {
    mnenomic : "CLR",
    exec: default_exec,
};

static DAA : Op = Op {
    mnenomic : "DAA",
    exec: default_exec,
};

static DEC : Op = Op {
    mnenomic : "DEC",
    exec: default_exec,
};

static EXG : Op = Op {
    mnenomic : "EXG",
    exec: default_exec,
};

static INC : Op = Op {
    mnenomic : "INC",
    exec: default_exec,
};

static JMP : Op = Op {
    mnenomic : "JMP",
    exec: default_exec,
};

static LBRA : Op = Op {
    mnenomic : "LBRA",
    exec: default_exec,
};

static LBSR : Op = Op {
    mnenomic : "LBSR",
    exec: default_exec,
};

static LSL_ASL : Op = Op {
    mnenomic : "LSL_ASL",
    exec: default_exec,
};

static NOP : Op = Op {
    mnenomic : "NOP",
    exec: default_exec,
};

static ORCC : Op = Op {
    mnenomic : "ORCC",
    exec: default_exec,
};

static PAGE1_OP : Op = Op {
    mnenomic : "PAGE1",
    exec: default_exec,
};

static PAGE2_OP : Op = Op {
    mnenomic : "PAGE2",
    exec: default_exec,
};

static ROL : Op = Op {
    mnenomic : "ROL",
    exec: default_exec,
};

static SEX : Op = Op {
    mnenomic : "SEX",
    exec: default_exec,
};

static SYNC : Op = Op {
    mnenomic : "SYNC",
    exec: default_exec,
};

static TFR : Op = Op {
    mnenomic : "TFR",
    exec: default_exec,
};

static TST : Op = Op {
    mnenomic : "TST",
    exec: default_exec,
};

static ABX : Op = Op {
    mnenomic : "ABX",
    exec: default_exec,
};

static ANDA : Op = Op {
    mnenomic : "ANDA",
    exec: default_exec,
};

static ASRA : Op = Op {
    mnenomic : "ASRA",
    exec: default_exec,
};

static ASRB : Op = Op {
    mnenomic : "ASRB",
    exec: default_exec,
};

static BGT : Op = Op {
    mnenomic : "BGT",
    exec: default_exec,
};

static BITA : Op = Op {
    mnenomic : "BITA",
    exec: default_exec,
};

static BLE : Op = Op {
    mnenomic : "BLE",
    exec: default_exec,
};

static BLT : Op = Op {
    mnenomic : "BLT",
    exec: default_exec,
};

static CLRA : Op = Op {
    mnenomic : "CLRA",
    exec: default_exec,
};

static CLRB : Op = Op {
    mnenomic : "CLRB",
    exec: default_exec,
};

static CMPA : Op = Op {
    mnenomic : "CMPA",
    exec: default_exec,
};

static COMA : Op = Op {
    mnenomic : "COMA",
    exec: default_exec,
};

static COMB : Op = Op {
    mnenomic : "COMB",
    exec: default_exec,
};

static CWAI : Op = Op {
    mnenomic : "CWAI",
    exec: default_exec,
};

static DECA : Op = Op {
    mnenomic : "DECA",
    exec: default_exec,
};

static DECB : Op = Op {
    mnenomic : "DECB",
    exec: default_exec,
};

static EORA : Op = Op {
    mnenomic : "EORA",
    exec: default_exec,
};

static INCA : Op = Op {
    mnenomic : "INCA",
    exec: default_exec,
};

static INCB : Op = Op {
    mnenomic : "INCB",
    exec: default_exec,
};

static LDA : Op = Op {
    mnenomic : "LDA",
    exec: default_exec,
};

static LEAS : Op = Op {
    mnenomic : "LEAS",
    exec: default_exec,
};

static LEAU : Op = Op {
    mnenomic : "LEAU",
    exec: default_exec,
};

static LEAX : Op = Op {
    mnenomic : "LEAX",
    exec: default_exec,
};

static LEAY : Op = Op {
    mnenomic : "LEAY",
    exec: default_exec,
};

static LSLA_ASLA : Op = Op {
    mnenomic : "LSLA_ASLA",
    exec: default_exec,
};

static LSLB_ASLB : Op = Op {
    mnenomic : "LSLB_ASLB",
    exec: default_exec,
};

static LSRA : Op = Op {
    mnenomic : "LSRA",
    exec: default_exec,
};

static LSRB : Op = Op {
    mnenomic : "LSRB",
    exec: default_exec,
};

static MUL : Op = Op {
    mnenomic : "MUL",
    exec: default_exec,
};

static NEGA : Op = Op {
    mnenomic : "NEGA",
    exec: default_exec,
};

static NEGB : Op = Op {
    mnenomic : "NEGB",
    exec: default_exec,
};

static PSHS : Op = Op {
    mnenomic : "PSHS",
    exec: default_exec,
};

static PSHU : Op = Op {
    mnenomic : "PSHU",
    exec: default_exec,
};

static PULS : Op = Op {
    mnenomic : "PULS",
    exec: default_exec,
};

static PULU : Op = Op {
    mnenomic : "PULU",
    exec: default_exec,
};

static RESET : Op = Op {
    mnenomic : "RESET",
    exec: default_exec,
};

static ROLA : Op = Op {
    mnenomic : "ROLA",
    exec: default_exec,
};

static ROLB : Op = Op {
    mnenomic : "ROLB",
    exec: default_exec,
};

static RORA : Op = Op {
    mnenomic : "RORA",
    exec: default_exec,
};

static RORB : Op = Op {
    mnenomic : "RORB",
    exec: default_exec,
};

static RTI : Op = Op {
    mnenomic : "RTI",
    exec: default_exec,
};

static RTS : Op = Op {
    mnenomic : "RTS",
    exec: default_exec,
};

static SBCA : Op = Op {
    mnenomic : "SBCA",
    exec: default_exec,
};

static SUBA : Op = Op {
    mnenomic : "SUBA",
    exec: default_exec,
};

static SUBD : Op = Op {
    mnenomic : "SUBD",
    exec: default_exec,
};

static SWI : Op = Op {
    mnenomic : "SWI",
    exec: default_exec,
};

static TSTA : Op = Op {
    mnenomic : "TSTA",
    exec: default_exec,
};

static TSTB : Op = Op {
    mnenomic : "TSTB",
    exec: default_exec,
};

static ADCA : Op = Op {
    mnenomic : "ADCA",
    exec: default_exec,
};

static ADCB : Op = Op {
    mnenomic : "ADCB",
    exec: default_exec,
};

static ADDA : Op = Op {
    mnenomic : "ADDA",
    exec: default_exec,
};

static ADDB : Op = Op {
    mnenomic : "ADDB",
    exec: default_exec,
};

static ADDD : Op = Op {
    mnenomic : "ADDD",
    exec: default_exec,
};

static ANDB : Op = Op {
    mnenomic : "ANDB",
    exec: default_exec,
};

static BITB : Op = Op {
    mnenomic : "BITB",
    exec: default_exec,
};

static BSR : Op = Op {
    mnenomic : "BSR",
    exec: default_exec,
};

static CMPB : Op = Op {
    mnenomic : "CMPB",
    exec: default_exec,
};


static EORB : Op = Op {
    mnenomic : "EORB",
    exec: default_exec,
};

static JSR : Op = Op {
    mnenomic : "JSR",
    exec: default_exec,
};

static LDB : Op = Op {
    mnenomic : "LDB",
    exec: default_exec,
};

static LDD : Op = Op {
    mnenomic : "LDD",
    exec: default_exec,
};

static LDU : Op = Op {
    mnenomic : "LDU",
    exec: default_exec,
};

static ORA : Op = Op {
    mnenomic : "ORA",
    exec: default_exec,
};

static ORB : Op = Op {
    mnenomic : "ORB",
    exec: default_exec,
};

static SBCB : Op = Op {
    mnenomic : "SBCB",
    exec: default_exec,
};

static STA : Op = Op {
    mnenomic : "STA",
    exec: default_exec,
};

static STB : Op = Op {
    mnenomic : "STB",
    exec: default_exec,
};

static STD : Op = Op {
    mnenomic : "STD",
    exec: default_exec,
};

static STU : Op = Op {
    mnenomic : "STU",
    exec: default_exec,
};

static SUBB : Op = Op {
    mnenomic : "SUBB",
    exec: default_exec,
};

static CMPX : Op = Op {
    mnenomic : "CMPX",
    exec: default_exec,
};

static LDX : Op = Op {
    mnenomic : "LDX",
    exec: default_exec,
};

static STX : Op = Op {
    mnenomic : "STX",
    exec: default_exec,
};

static CMPD : Op = Op {
    mnenomic: "CMPD",
    exec: default_exec,
};

static CMPY : Op = Op {
    mnenomic: "CMPY",
    exec: default_exec,
};

static LBEQ : Op = Op {
    mnenomic: "LBEQ",
    exec: default_exec,
};

static LBGE : Op = Op {
    mnenomic: "LBGE",
    exec: default_exec,
};

static LBGT : Op = Op {
    mnenomic: "LBGT",
    exec: default_exec,
};

static LBHI : Op = Op {
    mnenomic: "LBHI",
    exec: default_exec,
};

static LBHS_LBCC : Op = Op {
    mnenomic: "LBHS_LBCC",
    exec: default_exec,
};

static LBLE : Op = Op {
    mnenomic: "LBLE",
    exec: default_exec,
};

static LBLO_LBCS : Op = Op {
    mnenomic: "LBLO_LBCS",
    exec: default_exec,
};

static LBLS : Op = Op {
    mnenomic: "LBLS",
    exec: default_exec,
};

static LBLT : Op = Op {
    mnenomic: "LBLT",
    exec: default_exec,
};

static LBMI : Op = Op {
    mnenomic: "LBMI",
    exec: default_exec,
};

static LBNE : Op = Op {
    mnenomic: "LBNE",
    exec: default_exec,
};

static LBPL : Op = Op {
    mnenomic: "LBPL",
    exec: default_exec,
};

static LBRN : Op = Op {
    mnenomic: "LBRN",
    exec: default_exec,
};

static LBVC : Op = Op {
    mnenomic: "LBVC",
    exec: default_exec,
};

static LBVS : Op = Op {
    mnenomic: "LBVS",
    exec: default_exec,
};

static LDS : Op = Op {
    mnenomic: "LDS",
    exec: default_exec,
};

static LDY : Op = Op {
    mnenomic: "LDY",
    exec: default_exec,
};

static STS : Op = Op {
    mnenomic: "STS",
    exec: default_exec,
};

static STY : Op = Op {
    mnenomic: "STY",
    exec: default_exec,
};

static SWI2 : Op = Op {
    mnenomic: "SWI2",
    exec: default_exec,
};

static SWI3 : Op = Op {
    mnenomic: "SWI3",
    exec: default_exec,
};

static CMPU : Op = Op {
    mnenomic: "CMPU",
    exec: default_exec,
};

static CMPS : Op = Op {
    mnenomic: "CMPS",
    exec: default_exec,
};

////////////////////////////////////////////////////////////////////////////////

macro_rules! ins {
    ($op_code:expr, $op:ident, $am:ident, $cycles:expr, $bytes:expr, $operand_offset:expr) => ( Ins {
        op : &$op,
        addr_mode : &$am,
        op_code : $op_code,
        cycles : $cycles,
        bytes: $bytes,
        operand_offset: $operand_offset
    });
}


static INS : &'static [Ins] = &[
    ins!(0x00, NEG, DIRECT, 6, 2 , 1),
    ins!(0x01, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x02, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x03, COM, DIRECT, 6, 2 , 1),
    ins!(0x04, LSR, DIRECT, 6, 2 , 1),
    ins!(0x05, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x06, ROR, DIRECT, 6, 2 , 1),
    ins!(0x07, ASR, DIRECT, 6, 2 , 1),
    ins!(0x08, LSL_ASL, DIRECT, 6, 2 , 1),
    ins!(0x09, ROL, DIRECT, 6, 2 , 1),
    ins!(0x0A, DEC, DIRECT, 6, 2 , 1),
    ins!(0x0B, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x0C, INC, DIRECT, 6, 2 , 1),
    ins!(0x0D, TST, DIRECT, 6, 2 , 1),
    ins!(0x0E, JMP, DIRECT, 3, 2 , 1),
    ins!(0x0F, CLR, DIRECT, 6, 2 , 1),
    ins!(0x10, PAGE1_OP, VARIANT, 1, 1 , 1),
    ins!(0x11, PAGE2_OP, VARIANT, 1, 1 , 1),
    ins!(0x12, NOP, INHERENT, 2, 1 , 1),
    ins!(0x13, SYNC, INHERENT, 2, 1 , 1),
    ins!(0x14, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x15, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x16, LBRA, RELATIVE, 5, 3 , 1),
    ins!(0x17, LBSR, RELATIVE, 9, 3 , 1),
    ins!(0x18, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x19, DAA, INHERENT, 2, 1 , 1),
    ins!(0x1A, ORCC, IMMEDIATE, 3, 2 , 1),
    ins!(0x1B, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x1C, ANDCC, IMMEDIATE, 3, 2 , 1),
    ins!(0x1D, SEX, INHERENT, 2, 1 , 1),
    ins!(0x1E, EXG, INHERENT, 8, 2 , 1),
    ins!(0x1F, TFR, INHERENT, 7, 2 , 1),
    ins!(0x20, BRA, RELATIVE, 3, 2 , 1),
    ins!(0x21, BRN, RELATIVE, 3, 2 , 1),
    ins!(0x22, BHI, RELATIVE, 3, 2 , 1),
    ins!(0x23, BLS, RELATIVE, 3, 2 , 1),
    ins!(0x24, BHS_BCC, RELATIVE, 3, 2 , 1),
    ins!(0x25, BLO_BCS, RELATIVE, 3, 2 , 1),
    ins!(0x26, BNE, RELATIVE, 3, 2 , 1),
    ins!(0x27, BEQ, RELATIVE, 3, 2 , 1),
    ins!(0x28, BVC, RELATIVE, 3, 2 , 1),
    ins!(0x29, BVS, RELATIVE, 3, 2 , 1),
    ins!(0x2A, BPL, RELATIVE, 3, 2 , 1),
    ins!(0x2B, BMI, RELATIVE, 3, 2 , 1),
    ins!(0x2C, BGE, RELATIVE, 3, 2 , 1),
    ins!(0x2D, BLT, RELATIVE, 3, 2 , 1),
    ins!(0x2E, BGT, RELATIVE, 3, 2 , 1),
    ins!(0x2F, BLE, RELATIVE, 3, 2 , 1),
    ins!(0x30, LEAX, INDEXED, 4, 2 , 1),
    ins!(0x31, LEAY, INDEXED, 4, 2 , 1),
    ins!(0x32, LEAS, INDEXED, 4, 2 , 1),
    ins!(0x33, LEAU, INDEXED, 4, 2 , 1),
    ins!(0x34, PSHS, INHERENT, 5, 2 , 1),
    ins!(0x35, PULS, INHERENT, 5, 2 , 1),
    ins!(0x36, PSHU, INHERENT, 5, 2 , 1),
    ins!(0x37, PULU, INHERENT, 5, 2 , 1),
    ins!(0x38, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x39, RTS, INHERENT, 5, 1 , 1),
    ins!(0x3A, ABX, INHERENT, 3, 1 , 1),
    ins!(0x3B, RTI, INHERENT, 6, 1 , 1),   // *!!!!
    ins!(0x3C, CWAI, INHERENT, 21, 2 , 1),
    ins!(0x3D, MUL, INHERENT, 11, 1 , 1),
    ins!(0x3E, RESET, INHERENT, 1, 1 , 1), // *!!!
    ins!(0x3F, SWI, INHERENT, 19, 1 , 1),
    ins!(0x40, NEGA, INHERENT, 2, 1 , 1),
    ins!(0x41, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x42, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x43, COMA, INHERENT, 2, 1 , 1),
    ins!(0x44, LSRA, INHERENT, 2, 1 , 1),
    ins!(0x45, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x46, RORA, INHERENT, 2, 1 , 1),
    ins!(0x47, ASRA, INHERENT, 2, 1 , 1),
    ins!(0x48, LSLA_ASLA, INHERENT, 2, 1 , 1),
    ins!(0x49, ROLA, INHERENT, 2, 1 , 1),
    ins!(0x4A, DECA, INHERENT, 2, 1 , 1),
    ins!(0x4B, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x4C, INCA, INHERENT, 2, 1 , 1),
    ins!(0x4D, TSTA, INHERENT, 2, 1 , 1),
    ins!(0x4E, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x4F, CLRA, INHERENT, 2, 1 , 1),
    ins!(0x50, NEGB, INHERENT, 2, 1 , 1),
    ins!(0x51, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x52, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x53, COMB, INHERENT, 2, 1 , 1),
    ins!(0x54, LSRB, INHERENT, 2, 1 , 1),
    ins!(0x55, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x56, RORB, INHERENT, 2, 1 , 1),
    ins!(0x57, ASRB, INHERENT, 2, 1 , 1),
    ins!(0x58, LSLB_ASLB, INHERENT, 2, 1 , 1),
    ins!(0x59, ROLB, INHERENT, 2, 1 , 1),
    ins!(0x5A, DECB, INHERENT, 2, 1 , 1),
    ins!(0x5B, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x5C, INCB, INHERENT, 2, 1 , 1),
    ins!(0x5D, TSTB, INHERENT, 2, 1 , 1),
    ins!(0x5E, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x5F, CLRB, INHERENT, 2, 1 , 1),
    ins!(0x60, NEG, INDEXED, 6, 2 , 1),
    ins!(0x61, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x62, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x63, COM, INDEXED, 6, 2 , 1),
    ins!(0x64, LSR, INDEXED, 6, 2 , 1),
    ins!(0x65, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x66, ROR, INDEXED, 6, 2 , 1),
    ins!(0x67, ASR, INDEXED, 6, 2 , 1),
    ins!(0x68, LSL_ASL, INDEXED, 6, 2 , 1),
    ins!(0x69, ROL, INDEXED, 6, 2 , 1),
    ins!(0x6A, DEC, INDEXED, 6, 2 , 1),
    ins!(0x6B, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x6C, INC, INDEXED, 6, 2 , 1),
    ins!(0x6D, TST, INDEXED, 6, 2 , 1),
    ins!(0x6E, JMP, INDEXED, 3, 2 , 1),
    ins!(0x6F, CLR, INDEXED, 6, 2 , 1),
    ins!(0x70, NEG, EXTENDED, 7, 3 , 1),
    ins!(0x71, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x72, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x73, COM, EXTENDED, 7, 3 , 1),
    ins!(0x74, LSR, EXTENDED, 7, 3 , 1),
    ins!(0x75, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x76, ROR, EXTENDED, 7, 3 , 1),
    ins!(0x77, ASR, EXTENDED, 7, 3 , 1),
    ins!(0x78, LSL_ASL, EXTENDED, 7, 3 , 1),
    ins!(0x79, ROL, EXTENDED, 7, 3 , 1),
    ins!(0x7A, DEC, EXTENDED, 7, 3 , 1),
    ins!(0x7B, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x7C, INC, EXTENDED, 7, 3 , 1),
    ins!(0x7D, TST, EXTENDED, 7, 3 , 1),
    ins!(0x7E, JMP, EXTENDED, 3, 3 , 1),
    ins!(0x7F, CLR, EXTENDED, 7, 3 , 1),
    ins!(0x80, SUBA, IMMEDIATE, 2, 2 , 1),
    ins!(0x81, CMPA, IMMEDIATE, 2, 2 , 1),
    ins!(0x82, SBCA, IMMEDIATE, 2, 2 , 1),
    ins!(0x83, SUBD, IMMEDIATE, 4, 3 , 1),
    ins!(0x84, ANDA, IMMEDIATE, 2, 2 , 1),
    ins!(0x85, BITA, IMMEDIATE, 2, 2 , 1),
    ins!(0x86, LDA, IMMEDIATE, 2, 2 , 1),
    ins!(0x87, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x88, EORA, IMMEDIATE, 2, 2 , 1),
    ins!(0x89, ADCA, IMMEDIATE, 2, 2 , 1),
    ins!(0x8A, ORA, IMMEDIATE, 2, 2 , 1),
    ins!(0x8B, ADDA, IMMEDIATE, 2, 2 , 1),
    ins!(0x8C, CMPX, IMMEDIATE, 4, 3 , 1),
    ins!(0x8D, BSR, RELATIVE, 7, 2 , 1),
    ins!(0x8E, LDX, IMMEDIATE, 3, 3 , 1),
    ins!(0x8F, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0x90, SUBA, DIRECT, 4, 2 , 1),
    ins!(0x91, CMPA, DIRECT, 4, 2 , 1),
    ins!(0x92, SBCA, DIRECT, 4, 2 , 1),
    ins!(0x93, SUBD, DIRECT, 6, 2 , 1),
    ins!(0x94, ANDA, DIRECT, 4, 2 , 1),
    ins!(0x95, BITA, DIRECT, 4, 2 , 1),
    ins!(0x96, LDA, DIRECT, 4, 2 , 1),
    ins!(0x97, STA, DIRECT, 4, 2 , 1),
    ins!(0x98, EORA, DIRECT, 4, 2 , 1),
    ins!(0x99, ADCA, DIRECT, 4, 2 , 1),
    ins!(0x9A, ORA, DIRECT, 4, 2 , 1),
    ins!(0x9B, ADDA, DIRECT, 4, 2 , 1),
    ins!(0x9C, CMPX, DIRECT, 6, 2 , 1),
    ins!(0x9D, JSR, DIRECT, 7, 2 , 1),
    ins!(0x9E, LDX, DIRECT, 5, 2 , 1),
    ins!(0x9F, STX, DIRECT, 5, 2 , 1),
    ins!(0xA0, SUBA, INDEXED, 4, 2 , 1),
    ins!(0xA1, CMPA, INDEXED, 4, 2 , 1),
    ins!(0xA2, SBCA, INDEXED, 4, 2 , 1),
    ins!(0xA3, SUBD, INDEXED, 6, 2 , 1),
    ins!(0xA4, ANDA, INDEXED, 4, 2 , 1),
    ins!(0xA5, BITA, INDEXED, 4, 2 , 1),
    ins!(0xA6, LDA, INDEXED, 4, 2 , 1),
    ins!(0xA7, STA, INDEXED, 4, 2 , 1),
    ins!(0xA8, EORA, INDEXED, 4, 2 , 1),
    ins!(0xA9, ADCA, INDEXED, 4, 2 , 1),
    ins!(0xAA, ORA, INDEXED, 4, 2 , 1),
    ins!(0xAB, ADDA, INDEXED, 4, 2 , 1),
    ins!(0xAC, CMPX, INDEXED, 6, 2 , 1),
    ins!(0xAD, JSR, INDEXED, 7, 2 , 1),
    ins!(0xAE, LDX, INDEXED, 5, 2 , 1),
    ins!(0xAF, STX, INDEXED, 5, 2 , 1),
    ins!(0xB0, SUBA, EXTENDED, 5, 3 , 1),
    ins!(0xB1, CMPA, EXTENDED, 5, 3 , 1),
    ins!(0xB2, SBCA, EXTENDED, 5, 3 , 1),
    ins!(0xB3, SUBD, EXTENDED, 7, 3 , 1),
    ins!(0xB4, ANDA, EXTENDED, 5, 3 , 1),
    ins!(0xB5, BITA, EXTENDED, 5, 3 , 1),
    ins!(0xB6, LDA, EXTENDED, 5, 3 , 1),
    ins!(0xB7, STA, EXTENDED, 5, 3 , 1),
    ins!(0xB8, EORA, EXTENDED, 5, 3 , 1),
    ins!(0xB9, ADCA, EXTENDED, 5, 3 , 1),
    ins!(0xBA, ORA, EXTENDED, 5, 3 , 1),
    ins!(0xBB, ADDA, EXTENDED, 5, 3 , 1),
    ins!(0xBC, CMPX, EXTENDED, 7, 3 , 1),
    ins!(0xBD, JSR, EXTENDED, 8, 3 , 1),
    ins!(0xBE, LDX, EXTENDED, 6, 3 , 1),
    ins!(0xBF, STX, EXTENDED, 6, 3 , 1),
    ins!(0xC0, SUBB, IMMEDIATE, 2, 2 , 1),
    ins!(0xC1, CMPB, IMMEDIATE, 2, 2 , 1),
    ins!(0xC2, SBCB, IMMEDIATE, 2, 2 , 1),
    ins!(0xC3, ADDD, IMMEDIATE, 4, 3 , 1),
    ins!(0xC4, ANDB, IMMEDIATE, 2, 2 , 1),
    ins!(0xC5, BITB, IMMEDIATE, 2, 2 , 1),
    ins!(0xC6, LDB, IMMEDIATE, 2, 2 , 1),
    ins!(0xC7, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0xC8, EORB, IMMEDIATE, 2, 2 , 1),
    ins!(0xC9, ADCB, IMMEDIATE, 2, 2 , 1),
    ins!(0xCA, ORB, IMMEDIATE, 2, 2 , 1),
    ins!(0xCB, ADDB, IMMEDIATE, 2, 2 , 1),
    ins!(0xCC, LDD, IMMEDIATE, 3, 3 , 1),
    ins!(0xCD, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0xCE, LDU, IMMEDIATE, 3, 3 , 1),
    ins!(0xCF, ILLEGAL, ILLEGAL_ADDR, 1, 1 , 1),
    ins!(0xD0, SUBB, DIRECT, 4, 2 , 1),
    ins!(0xD1, CMPB, DIRECT, 4, 2 , 1),
    ins!(0xD2, SBCB, DIRECT, 4, 2 , 1),
    ins!(0xD3, ADDD, DIRECT, 6, 2 , 1),
    ins!(0xD4, ANDB, DIRECT, 4, 2 , 1),
    ins!(0xD5, BITB, DIRECT, 4, 2 , 1),
    ins!(0xD6, LDB, DIRECT, 4, 2 , 1),
    ins!(0xD7, STB, DIRECT, 4, 2 , 1),
    ins!(0xD8, EORB, DIRECT, 4, 2 , 1),
    ins!(0xD9, ADCB, DIRECT, 4, 2 , 1),
    ins!(0xDA, ORB, DIRECT, 4, 2 , 1),
    ins!(0xDB, ADDB, DIRECT, 4, 2 , 1),
    ins!(0xDC, LDD, DIRECT, 5, 2 , 1),
    ins!(0xDD, STD, DIRECT, 5, 2 , 1),
    ins!(0xDE, LDU, DIRECT, 5, 2 , 1),
    ins!(0xDF, STU, DIRECT, 5, 2 , 1),
    ins!(0xE0, SUBB, INDEXED, 4, 2 , 1),
    ins!(0xE1, CMPB, INDEXED, 4, 2 , 1),
    ins!(0xE2, SBCB, INDEXED, 4, 2 , 1),
    ins!(0xE3, ADDD, INDEXED, 6, 2 , 1),
    ins!(0xE4, ANDB, INDEXED, 4, 2 , 1),
    ins!(0xE5, BITB, INDEXED, 4, 2 , 1),
    ins!(0xE6, LDB, INDEXED, 4, 2 , 1),
    ins!(0xE7, STB, INDEXED, 4, 2 , 1),
    ins!(0xE8, EORB, INDEXED, 4, 2 , 1),
    ins!(0xE9, ADCB, INDEXED, 4, 2 , 1),
    ins!(0xEA, ORB, INDEXED, 4, 2 , 1),
    ins!(0xEB, ADDB, INDEXED, 4, 2 , 1),
    ins!(0xEC, LDD, INDEXED, 5, 2 , 1),
    ins!(0xED, STD, INDEXED, 5, 2 , 1),
    ins!(0xEE, LDU, INDEXED, 5, 2 , 1),
    ins!(0xEF, STU, INDEXED, 5, 2 , 1),
    ins!(0xF0, SUBB, EXTENDED, 5, 3 , 1),
    ins!(0xF1, CMPB, EXTENDED, 5, 3 , 1),
    ins!(0xF2, SBCB, EXTENDED, 5, 3 , 1),
    ins!(0xF3, ADDD, EXTENDED, 7, 3 , 1),
    ins!(0xF4, ANDB, EXTENDED, 5, 3 , 1),
    ins!(0xF5, BITB, EXTENDED, 5, 3 , 1),
    ins!(0xF6, LDB, EXTENDED, 5, 3 , 1),
    ins!(0xF7, STB, EXTENDED, 5, 3 , 1),
    ins!(0xF8, EORB, EXTENDED, 5, 3 , 1),
    ins!(0xF9, ADCB, EXTENDED, 5, 3 , 1),
    ins!(0xFA, ORB, EXTENDED, 5, 3 , 1),
    ins!(0xFB, ADDB, EXTENDED, 5, 3 , 1),
    ins!(0xFC, LDD, EXTENDED, 6, 3 , 1),
    ins!(0xFD, STD, EXTENDED, 6, 3 , 1),
    ins!(0xFE, LDU, EXTENDED, 6, 3 , 1),
    ins!(0xFF, STU, EXTENDED, 6, 3 , 1),
    ];

static SWI3_INS_INHERENT:    &'static Ins = &ins!( 0x113F, SWI3, INHERENT     ,  20   ,   2   , 2);
static CMPU_INS_IMMEDIATE:   &'static Ins = &ins!( 0x1183, CMPU, IMMEDIATE    ,   5   ,   4   , 2);
static CMPS_INS_IMMEDIATE:   &'static Ins = &ins!( 0x118C, CMPS, IMMEDIATE    ,   5   ,   4   , 2);
static CMPU_INS_DIRECT:      &'static Ins = &ins!( 0x1193, CMPU, DIRECT       ,   7   ,   3   , 2);
static CMPS_INS_DIRECT:      &'static Ins = &ins!( 0x119C, CMPS, DIRECT       ,   7   ,   3   , 2);
static CMPU_INS_INDEXED:     &'static Ins = &ins!( 0x11A3, CMPU, INDEXED      ,   7   ,   3   , 2);
static CMPS_INS_INDEXED:     &'static Ins = &ins!( 0x11AC, CMPS, INDEXED      ,   7   ,   3   , 2);
static CMPU_INS_EXTENDED:    &'static Ins = &ins!( 0x11B3, CMPU, EXTENDED     ,   8   ,   4   , 2);
static CMPS_INS_EXTENDED:    &'static Ins = &ins!( 0x11BC, CMPS, EXTENDED     ,   8   ,   4   , 2);
static ILLEGAL_A11:    &'static Ins = &ins!( 0x11BC, ILLEGAL, INHERENT     ,   1   ,   1   , 2);

fn get_ins_a11(op : u16) -> &'static Ins {
    match op {
        0x113F => SWI3_INS_INHERENT,
        0x1183 => CMPU_INS_IMMEDIATE,
        0x118C => CMPS_INS_IMMEDIATE,
        0x1193 => CMPU_INS_DIRECT,
        0x119C => CMPS_INS_DIRECT,
        0x11A3 => CMPU_INS_INDEXED,
        0x11AC => CMPS_INS_INDEXED,
        0x11B3 => CMPU_INS_EXTENDED,
        0x11BC => CMPS_INS_EXTENDED,
        _ => ILLEGAL_A11,
    }
}

static LBRN_RELATIVE: &'static Ins      =    &ins!( 0x1021, LBRN, RELATIVE, 5, 4 , 2);
static LBHI_RELATIVE: &'static Ins      =    &ins!( 0x1022, LBHI, RELATIVE, 5, 4 , 2);
static LBLS_RELATIVE: &'static Ins      =    &ins!( 0x1023, LBLS, RELATIVE, 5, 4 , 2);
static LBHS_LBCC_RELATIVE: &'static Ins =  &ins!( 0x1024, LBHS_LBCC, RELATIVE, 5, 4 , 2);
static LBLO_LBCS_RELATIVE: &'static Ins =  &ins!( 0x1025, LBLO_LBCS, RELATIVE, 5, 4 , 2);
static LBNE_RELATIVE: &'static Ins      =    &ins!( 0x1026, LBNE, RELATIVE, 5, 4 , 2);
static LBEQ_RELATIVE: &'static Ins      =    &ins!( 0x1027, LBEQ, RELATIVE, 5, 4 , 2);
static LBVC_RELATIVE: &'static Ins      =    &ins!( 0x1028, LBVC, RELATIVE, 5, 4 , 2);
static LBVS_RELATIVE: &'static Ins      =    &ins!( 0x1029, LBVS, RELATIVE, 5, 4 , 2);
static LBPL_RELATIVE: &'static Ins      =    &ins!( 0x102A, LBPL, RELATIVE, 5, 4 , 2);
static LBMI_RELATIVE: &'static Ins      =    &ins!( 0x102B, LBMI, RELATIVE, 5, 4 , 2);
static LBGE_RELATIVE: &'static Ins      =    &ins!( 0x102C, LBGE, RELATIVE, 5, 4 , 2);
static LBLT_RELATIVE: &'static Ins      =    &ins!( 0x102D, LBLT, RELATIVE, 5, 4 , 2);
static LBGT_RELATIVE: &'static Ins      =    &ins!( 0x102E, LBGT, RELATIVE, 5, 4 , 2);
static LBLE_RELATIVE: &'static Ins      =    &ins!( 0x102F, LBLE, RELATIVE, 5, 4 , 2);
static SWI2_INHERENT: &'static Ins      =    &ins!( 0x103F, SWI2, INHERENT, 20, 2 , 2);
static CMPD_IMMEDIATE: &'static Ins     =    &ins!( 0x1083, CMPD, IMMEDIATE, 5, 4 , 2);
static CMPY_IMMEDIATE: &'static Ins     =    &ins!( 0x108C, CMPY, IMMEDIATE, 5, 4 , 2);
static LDY_IMMEDIATE: &'static Ins      =    &ins!( 0x108E, LDY, IMMEDIATE, 4, 4 , 2);
static CMPD_DIRECT: &'static Ins        =    &ins!( 0x1093, CMPD, DIRECT, 7, 3 , 2);
static CMPY_DIRECT: &'static Ins        =    &ins!( 0x109C, CMPY, DIRECT, 7, 3 , 2);
static LDY_DIRECT: &'static Ins         =    &ins!( 0x109E, LDY, DIRECT, 6, 3 , 2);
static STY_DIRECT: &'static Ins         =    &ins!( 0x109F, STY, DIRECT, 6, 3 , 2);
static CMPD_INDEXED: &'static Ins       =    &ins!( 0x10A3, CMPD, INDEXED, 7, 3 , 2);
static CMPY_INDEXED: &'static Ins       =    &ins!( 0x10AC, CMPY, INDEXED, 7, 3 , 2);
static LDY_INDEXED: &'static Ins        =    &ins!( 0x10AE, LDY, INDEXED, 6, 3 , 2);
static STY_INDEXED: &'static Ins        =    &ins!( 0x10AF, STY, INDEXED, 6, 3 , 2);
static CMPD_EXTENDED: &'static Ins      =    &ins!( 0x10B3, CMPD, EXTENDED, 8, 4 , 2);
static CMPY_EXTENDED: &'static Ins      =    &ins!( 0x10BC, CMPY, EXTENDED, 8, 4 , 2);
static LDY_EXTENDED: &'static Ins       =    &ins!( 0x10BE, LDY, EXTENDED, 7, 4 , 2);
static STY_EXTENDED: &'static Ins       =    &ins!( 0x10BF, STY, EXTENDED, 7, 4 , 2);
static LDS_IMMEDIATE: &'static Ins      =    &ins!( 0x10CE, LDS, IMMEDIATE, 4, 4 , 2);
static LDS_DIRECT: &'static Ins         =    &ins!( 0x10DE, LDS, DIRECT, 6, 3 , 2);
static STS_DIRECT: &'static Ins         =    &ins!( 0x10DF, STS, DIRECT, 6, 3 , 2);
static LDS_INDEXED: &'static Ins        =    &ins!( 0x10EE, LDS, INDEXED, 6, 3 , 2);
static STS_INDEXED: &'static Ins        =    &ins!( 0x10EF, STS, INDEXED, 6, 3 , 2);
static LDS_EXTENDED: &'static Ins       =    &ins!( 0x10FE, LDS, EXTENDED, 7, 4 , 2);
static STS_EXTENDED: &'static Ins       =    &ins!( 0x10FF, STS, EXTENDED, 7, 4 , 2);
static ILLEGAL_A10:    &'static Ins = &ins!( 0x1000, ILLEGAL, INHERENT     ,   1   ,   1   , 2);

fn get_ins_a10(op : u16) -> &'static Ins {
    match op {
        0x1021=> LBRN_RELATIVE ,
        0x1022=>  LBHI_RELATIVE ,
        0x1023=>  LBLS_RELATIVE ,
        0x1024=>  LBHS_LBCC_RELATIVE ,
        0x1025=>  LBLO_LBCS_RELATIVE ,
        0x1026=>  LBNE_RELATIVE ,
        0x1027=>  LBEQ_RELATIVE ,
        0x1028=>  LBVC_RELATIVE ,
        0x1029=>  LBVS_RELATIVE ,
        0x102A=>  LBPL_RELATIVE ,
        0x102B=>  LBMI_RELATIVE ,
        0x102C=>  LBGE_RELATIVE ,
        0x102D=>  LBLT_RELATIVE ,
        0x102E=>  LBGT_RELATIVE ,
        0x102F=>  LBLE_RELATIVE ,
        0x103F=>  SWI2_INHERENT ,
        0x1083=>  CMPD_IMMEDIATE ,
        0x108C=>  CMPY_IMMEDIATE ,
        0x108E=>  LDY_IMMEDIATE ,
        0x1093=>  CMPD_DIRECT ,
        0x109C=>  CMPY_DIRECT ,
        0x109E=>  LDY_DIRECT ,
        0x109F=>  STY_DIRECT ,
        0x10A3=>  CMPD_INDEXED ,
        0x10AC=>  CMPY_INDEXED ,
        0x10AE=>  LDY_INDEXED ,
        0x10AF=>  STY_INDEXED ,
        0x10B3=>  CMPD_EXTENDED ,
        0x10BC=>  CMPY_EXTENDED ,
        0x10BE=>  LDY_EXTENDED ,
        0x10BF=>  STY_EXTENDED ,
        0x10CE=>  LDS_IMMEDIATE ,
        0x10DE=>  LDS_DIRECT ,
        0x10DF=>  STS_DIRECT ,
        0x10EE=>  LDS_INDEXED ,
        0x10EF=>  STS_INDEXED ,
        0x10FE=>  LDS_EXTENDED ,
        0x10FF=>  STS_EXTENDED ,
        _ => ILLEGAL_A10,
    }
}

pub fn get_ins(op : u16 ) -> &'static Ins {
    match op >> 8 {
        0x10 => get_ins_a10(op),
        0x11 => get_ins_a11(op),
        _ => &INS[op as usize],
    }
}

fn is_multi_byte( op_code : u8 ) -> bool {
    (op_code == 0x10 || op_code == 0x11)
}

pub fn decode_ins(addr : u16, mem : &MemMap) -> (&'static Ins, u16, u16) {

    let mut op = mem.load_byte(addr) as u16;

    let mut operand_offset = 1;

    if is_multi_byte(op as u8) {
        op = mem.load_word(addr) as u16 ;
        operand_offset = 2;
    }

    let ins = get_ins(op);

    let (operand, next_ins) = fetch_operand(&ins.addr_mode.mode, mem, addr + operand_offset);

    (ins, operand, next_ins)
}


