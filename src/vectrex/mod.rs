mod dac;

use mem::{MemMap};
use clap::{ArgMatches};
use cpu::{Cpu,Regs};
use clock::Clock;
use m6522::M6522;


pub struct Vectrex {
    mem : MemMap,
    cpu : Cpu,
    clock : Clock,
    m6522 : M6522,
    dac   : dac::Dac,
}

impl Vectrex {

    pub fn new() -> Vectrex {

        let regs = Regs::new();
        let cpu = Cpu::from_regs(regs);
        let mem = MemMap::new();
        let m6522 = M6522::new(0,4096);
        let clock = Clock::new(1_500_000);

        Vectrex {
            mem   : mem,
            cpu   : cpu,
            clock : clock,
            m6522 : m6522,
            dac   : dac::Dac {},
        }
    }

    pub fn from_matches(matches : &ArgMatches) -> Vectrex {
        Vectrex::new()
    }
}
