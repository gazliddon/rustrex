#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

extern crate regex;
#[macro_use] mod cpu;

mod mem;
mod via;
mod symtab;
mod utils;
mod diss;
mod proclog;

use proclog::{Step};
// use symtab::SymbolTable;
use mem::{MemoryIO, MemMap};
use cpu::{Cpu };

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

fn main() {
    use proclog::{read_step_log, read_step_log_lines};

    let log_file_name = "utils/6809/6809.log";

    let steps = read_step_log(log_file_name);
    let lines = read_step_log_lines(log_file_name);

    let mut mem = DEF_MACHINE.create_memmap();
    let mut cpu = Cpu::from_regs(&steps[0].regs);

    let mut cycles = 0;

    let mut step_i = 0;

    for log_step in steps {

        let sim_step = Step::from_sim(&mem, &cpu.regs, cycles);

        let ins = cpu.step(&mut mem);

        let comp = log_step.compare(&sim_step);

        println!("{} {:?}", log_step, log_step.regs.flags);
        println!("{} {:?}", sim_step, sim_step.regs.flags);
        println!("");

        if comp.regs == false {

            println!("log: {:?} {} {:?}", log_step.regs, log_step.regs.flags.bits(), log_step.regs.flags);
            println!("sim: {:?} {} {:?}", sim_step.regs, sim_step.regs.flags.bits(), sim_step.regs.flags );
            println!("");

            println!("{:?}", comp );

            panic!("fix this!")

        }

        cycles = cycles + ins.cycles;

        step_i = step_i + 1;
    }

}

