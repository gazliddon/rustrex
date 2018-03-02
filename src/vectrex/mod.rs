mod dac;

use mem::{MemMap, MemBlock, MemoryIO, MemMapIO};
 
use clap::{ArgMatches};
use cpu::{Cpu,Regs};
use cpu::StandardClock;
use m6522::M6522;
use sha1::Sha1;
use std::cell::RefCell;
use std::rc::Rc;


static FAST_ROM: &'static [u8] = include_bytes!("../../resources/fastrom.dat");
static SYS_ROM: &'static [u8]  = include_bytes!("../../resources/rom.dat");

pub struct Vectrex {
    mem   : MemMap,
    cpu   : Cpu,
    m6522 : M6522,
    dac   : dac::Dac,

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

impl MemoryIO for Vectrex {
    fn upload(&mut self, addr : u16, data : &[u8]) {
        panic!("TBD");
    }

    fn get_range(&self) -> (u16, u16) {
        panic!("TBD");
    }

    fn update_sha1(&self, digest : &mut Sha1) {
        panic!("TBD");
    }

    fn load_byte(&self, addr:u16) -> u8 {
        panic!("TBD");
    }
        
    fn store_byte(&mut self, addr:u16, val:u8) {
        panic!("TBD");
    }
    
    fn get_name(&self) -> &String {
        panic!("");
    }
}

impl Vectrex {

    pub fn new() -> Vectrex {

        let rc_clock = Rc::new(RefCell::new(StandardClock::new(1_500_000)));

        let cpu = Cpu::from_regs(&Regs::new());

        let mut mem = MemMap::new();

        let m6522 = M6522::new(0,4096, &rc_clock);

        mem.add_memory(mk_data_mem(0xe000,"sysrom", FAST_ROM, false));
        mem.add_mem_block("cart", false, 0, 16 * 1024);
        mem.add_mem_block("ram", true, 0xc800,  1024);

        Vectrex {
            mem   ,
            cpu   ,
            m6522 ,
            dac   : dac::Dac {},
            rc_clock,
        }
    }

    pub fn from_matches(matches : &ArgMatches) -> Vectrex {
        Vectrex::new()
    }

    pub fn step(&mut self) {

    }
}


