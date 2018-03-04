
use clap::{ArgMatches};
use std::cell::RefCell;
use std::rc::Rc;
use cpu::step;

use mem::{MemMap, MemBlock, MemoryIO, MemMapIO};
mod dac;
use cpu::{Regs, StandardClock};
use m6522::M6522;


static FAST_ROM: &'static [u8] = include_bytes!("../../resources/fastrom.dat");
static SYS_ROM: &'static [u8]  = include_bytes!("../../resources/rom.dat");

pub struct Vectrex {
    mem      : MemMap,
    m6522    : M6522,
    regs     : Regs,
    dac      : dac::Dac,
    rc_clock : Rc<RefCell<StandardClock>>,
}

impl MemBlock {
    pub fn from_data(addr : u16 ,name : &str, data : &[u8], writeable : bool ) -> MemBlock {

        let len = data.len() as u32;

        let last_byte = addr as u32 + len;

        assert!(last_byte < 0x1_0000);

        let mut r = MemBlock::new(name, writeable, addr, data.len() );

        r.data = data.to_vec();

        r
    }
}

fn mk_data_mem(addr : u16 ,name : &str, data : &[u8], writeable : bool ) -> Box<MemoryIO> {
    Box::new(MemBlock::from_data(addr, name, data, writeable))
}


impl Vectrex {

    pub fn new() -> Vectrex {

        let rc_clock = Rc::new(RefCell::new(StandardClock::new(1_500_000)));

        let mut mem = MemMap::new();

        let m6522 = M6522::new(0,4096, &rc_clock);

        mem.add_memory(mk_data_mem(0xe000,"sysrom", FAST_ROM, false));
        mem.add_mem_block("cart", false, 0, 16 * 1024);
        mem.add_mem_block("ram", true, 0xc800,  1024);

        Vectrex {
            mem, m6522,rc_clock,
            dac   : dac::Dac {},
            regs  : Regs::new(),
        }
    }

    pub fn from_matches(matches : &ArgMatches) -> Vectrex {
        Vectrex::new()
    }

    pub fn step(&mut self) {
        let ins = step(&mut self.regs, &mut self.mem, &self.rc_clock);

    }
}


