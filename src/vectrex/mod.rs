use clap::{ArgMatches};
use std::cell::RefCell;
use std::rc::Rc;

use mem::*;
mod dac;
use cpu::{Regs, StandardClock, step};
use m6522::M6522;

#[derive(Debug, Clone, Copy)]
enum MemRegion {
    Illegal,
    Ram,
    Rom,
    Cart,
    VIA,
}

static FAST_ROM: &'static [u8] = include_bytes!("../../resources/fastrom.dat");
static SYS_ROM: &'static [u8]  = include_bytes!("../../resources/rom.dat");

// Contains memory and memmapped perihpherals

struct VecMem {
    via    : M6522,
    dac      : dac::Dac,
    sys_rom  : MemBlock,
    cart_rom : MemBlock,
    ram      : MemBlock,
    addr_to_region : [MemRegion; 0x1_0000],
    name     : String,
}

impl VecMem {

    pub fn new(rc_clock : &Rc<RefCell<StandardClock>>) -> VecMem {

        let name = "VecMem".to_string();

        let via = M6522::new(0,4096, rc_clock);

        let sys_rom   = MemBlock::from_data(0xe000, "sys_rom", FAST_ROM, false);
        let cart_rom  = MemBlock::new("cart", true, 0, 16 * 1024);
        let ram       = MemBlock::new("ram", false, 0xc000, 1024);
        let dac       = dac::Dac {};

        VecMem {
            sys_rom, cart_rom, ram, dac, name,via,
            addr_to_region :  [MemRegion::Illegal; 0x1_0000],
        }
    }
}

impl MemoryIO for VecMem {

    fn upload(&mut self, addr : u16, data : &[u8]) {
        unimplemented!("TBD")
    }

    fn get_range(&self) -> (u16, u16) {
        (0 as u16, 0xffff as u16)
    }

    fn update_sha1(&self, digest : &mut Sha1) {
        unimplemented!("TBD")
    }

    fn load_byte(&self, addr:u16) -> u8 {
        let region = self.addr_to_region[addr as usize];

        use self::MemRegion::*;

        match region {
            Ram => self.ram.load_byte(addr),
            Rom => self.sys_rom.load_byte(addr),
            Cart => self.cart_rom.load_byte(addr),
            VIA => 0,
            Illegal => panic!("Illegal!"),
        }
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        let region = self.addr_to_region[addr as usize];
        use self::MemRegion::*;
        match region {
            Cart | Rom => panic!("Illegal wirte to rom"),
            Illegal => panic!("Illegal!"),
            Ram => self.ram.store_byte(addr,val),
            VIA => (),
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct Vectrex {
    mem      : MemMap,
    m6522    : M6522,
    regs     : Regs,
    dac      : dac::Dac,
    rc_clock : Rc<RefCell<StandardClock>>,
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


