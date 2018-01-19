#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

extern crate regex;

#[macro_use] extern crate lazy_static;
#[macro_use] mod cpu;

mod mem;
mod via;
mod symtab;
mod utils;
mod diss;
mod proclog;

use symtab::SymbolTable;
use diss::Disassembler;
use mem::{MemoryIO, MemMap};

use cpu::{Cpu, Flags};

// use mem::{MemoryIO};


////////////////////////////////////////////////////////////////////////////////
struct MemInit(&'static str, bool, u16, u16);
struct RomInit(&'static str, u16);

pub struct MachineInit {
    mem_regions : &'static [MemInit],
    roms : &'static[RomInit],
}

impl MachineInit {

    pub fn create_memmap(&self) -> MemMap {

        let mut m = MemMap::new();

        use utils::{load_file};

        for mb in self.mem_regions {
            m.add_mem_block(mb.0, mb.1, mb.2, mb.3)
        };

        for rom in self.roms {
            let data = load_file(rom.0);
            let addr= rom.1;
            m.upload(addr, &data);
        };
        m
    }
}

////////////////////////////////////////////////////////////////////////////////
static DEF_MACHINE: MachineInit = MachineInit {
    mem_regions: &[
        MemInit("cart"  , false, 0     , 0x8000 ),
        MemInit("sysrom", false, 0xe000, 0x2000),
        MemInit("ram"   , true , 0xc800, 0x800) 
    ],

    roms: &[
        RomInit( "resources/rom.dat", 0xe000 ),
        RomInit( "utils/6809/6809all.raw", 0x1000 ) ],
};

fn create_test_cpu() -> Cpu {

    let mut cpu = Cpu::new();

    {
        let r = &mut cpu.regs;

        r.set_d(0x44);

        r.pc = 0x1000;
        r.x  = 0xabab;
        r.y  = 0xaaf1;
        r.s  = 0x02e0;
        r.u  = 0x7f34;
        r.dp = 0x0000;

        r.flags.insert(Flags::E | Flags::Z);
    }

    cpu
}

fn main() {
    use proclog::read_step_log;
    let steps = read_step_log("utils/6809/6809.log");

    for s in steps {
        println!("{:?}", s)

    }

    if false {

        let syms = SymbolTable::new("resources/syms.yaml");

        let mut mm = DEF_MACHINE.create_memmap();
        let mut diss = Disassembler::new();

        let mut cpu = create_test_cpu();

        let mut cycles = 0;

        for i in 0..30 {

            let old_regs_str = format!("{}", cpu.regs);
            let mem_str = mm.get_mem_as_str(cpu.regs.pc, 5).to_lowercase();

            let (ins, txt) =  diss.diss(&mm, cpu.regs.pc,Some(&syms));

            let i = cpu.step(&mut mm);

            println!("{} {:16} {} {:>8}", old_regs_str, txt.to_uppercase(), mem_str, cycles);

            cycles = cycles + ins.cycles;
        }
    }




}

