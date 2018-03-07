use clap::{ArgMatches};
use std::cell::RefCell;
use std::rc::Rc;

use mem::*;
use cpu::{Regs, StandardClock, Clock, InstructionDecoder};
use cpu;

use m6522::M6522;

use vectrex::dac;

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
// decodes memory map

struct VecMem<C : Clock> {
    via            : M6522<C>,
    dac            : dac::Dac,
    sys_rom        : MemBlock,
    cart_rom       : MemBlock,
    ram            : MemBlock,
    addr_to_region : [MemRegion; 0x1_0000],
    name           : String,
}

fn build_addr_to_region(mem_tab :  &[(MemRegion, &MemoryIO )]) -> [MemRegion; 0x1_0000] {
    use self::MemRegion::*;

    let mut ret = [Illegal; 0x1_0000];

    for (i, id) in ret.iter_mut().enumerate() {
        for &(this_id, mem) in mem_tab {
            if mem.is_in_range(i as u16) {
                *id = this_id;
            }
        }
    }

    ret
}

impl<C: Clock> VecMem<C> {

    pub fn new(rc_clock : &Rc<RefCell<C>>) -> VecMem<C> {

        let name = "VecMem".to_string();

        let via = M6522::new(0xd000,0x800, rc_clock);

        let sys_rom   = MemBlock::from_data(0xe000, "sys_rom", FAST_ROM, false);
        let cart_rom  = MemBlock::new("cart", true, 0, 16 * 1024);
        let ram       = MemBlock::new("ram", false, 0xc800, 1024);

        let dac       = dac::Dac {};

        let addr_to_region = {

            use self::MemRegion::*;

            let mems : &[(MemRegion, &MemoryIO )] = &[
                (Rom, &sys_rom ),
                (Cart, &cart_rom ), 
                (Ram, &ram ),
                (VIA, &via) ];

            build_addr_to_region(mems)
        };

        VecMem {
            sys_rom, cart_rom, ram, dac, name,via, addr_to_region
        }
    }
}

impl<C : Clock> MemoryIO for VecMem<C> {

    fn upload(&mut self, addr : u16, data : &[u8]) {
        unimplemented!("TBD")
    }

    fn get_range(&self) -> (u16, u16) {
        (0, 0xffff)
    }

    fn update_sha1(&self, digest : &mut Sha1) {
        unimplemented!("TBD")
    }

    fn load_byte(&mut self, addr:u16) -> u8 {
        let region = self.addr_to_region[addr as usize];

        use self::MemRegion::*;

        match region {
            Ram     => self.ram.load_byte(addr),
            Rom     => self.sys_rom.load_byte(addr),
            Cart    => self.cart_rom.load_byte(addr),
            VIA     => self.via.load_byte(addr),
            Illegal => panic!("Illegal! read from {:02x}", addr),
        }
    }

    fn store_byte(&mut self, addr:u16, val:u8) {

        let region = self.addr_to_region[addr as usize];

        use self::MemRegion::*;

        match region {
            Cart | Rom => panic!("Illegal wirte to rom"),
            Illegal    => panic!("Illegal write of {}  to {:04X}", val, addr),
            Ram        => self.ram.store_byte(addr,val),
            VIA        => self.via.store_byte(addr,val),
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}




impl cpu::Host<VecMem<StandardClock>, StandardClock> for Vectrex {

    fn mem(&mut self) -> &mut VecMem<StandardClock> {
        &mut self.vec_mem
    }

    fn clock(&mut self) -> &Rc<RefCell< StandardClock >> {
        panic!("")
    }

    fn regs(&mut self) -> &mut Regs {
        &mut self.regs
    }
}


pub struct Vectrex {
    regs     : Regs,
    rc_clock : Rc<RefCell<StandardClock>>,
    vec_mem  : VecMem<StandardClock>,
}

fn mk_data_mem(addr : u16 ,name : &str, data : &[u8], writeable : bool ) -> Box<MemoryIO> {
    Box::new(MemBlock::from_data(addr, name, data, writeable))
}

impl Vectrex {

    pub fn new() -> Vectrex {
        let rc_clock = Rc::new(RefCell::new(StandardClock::new(1_500_000)));

        let vec_mem = VecMem::new(&rc_clock);

        let mut ret = Vectrex {
            rc_clock,
            regs  : Regs::new(),
            vec_mem,
        };

        ret.reset();

        ret
    }


    pub fn from_matches(matches : &ArgMatches) -> Vectrex {
        Vectrex::new()
    }

    pub fn step(&mut self) -> InstructionDecoder {
        // let mut diss = Disassembler::new();
        // let pc = self.regs.pc;
        // let (_, txt) =  diss.diss(&self.vec_mem, pc, None);

        // println!("${:04x}   {:20} : {} ",  pc, txt, self.regs);

        cpu::step(&mut self.regs, &mut self.vec_mem, &self.rc_clock)
    }

    pub fn reset(&mut self) {

        cpu::reset(&mut self.regs, &mut self.vec_mem);
    }
}


