/* 
 * Simple 6809 machine to test code on

    0000 -> 97ff Screen (304 * 256 pixels / 4bbpp)
    9800 -> 98ff IO
    9900 -> FFFF RAM 6700 (26k)

IO
    9800 -> 982F = Palette ram - 16 * RGB byte per col = 0x30]
    9830  raster pos
    9831  switches 1 
                b0 = Up
                b1 = Down
                b2 = Left
                b3 = Right
                b4 = Fire 1
                b5 = Fire 2
    9831  switches 2

*/

use simple::Io;
use clap::{ArgMatches};
use mem::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MemRegion {
    Illegal,
    Ram,
    IO,
    Screen,
}


struct SimpleMem {
    ram                : MemBlock,
    pub screen         : MemBlock,
    pub io             : Io,
    addr_to_region     : [MemRegion; 0x1_0000],
    name               : String,
}

impl SimpleMem {
    pub fn new() -> Self {

        let screen    = MemBlock::new("screen", false, 0x0000,0x9800);
        let ram       = MemBlock::new("ram", false, 0x9900, 0x1_0000 - 0x9900);
        let name      = "simple".to_string();
        let io        = Io::new();

        let addr_to_region = {

            use self::MemRegion::*;

            let mems : &[(MemRegion, &MemoryIO )] = &[
                (IO, &io),
                (Screen, &screen ),
                (Ram, &ram ), ];

            build_addr_to_region(Illegal, mems)
        };

        SimpleMem {
            ram,screen,name, addr_to_region, io
        }
    }

    fn get_region(&self, _addr : u16) -> &MemoryIO {
        let region = self.addr_to_region[_addr as usize];

        use self::MemRegion::*;

        match region {
            Ram       => &self.ram,
            IO        => &self.io,
            Screen    => &self.screen,
            Illegal   => panic!("Illegal! inspect from {:02x}", _addr),
        }
    }

    fn get_region_mut(&mut self, _addr : u16) -> &mut MemoryIO {
        let region = self.addr_to_region[_addr as usize];
        use self::MemRegion::*;

        match region {
            Ram       => &mut self.ram,
            IO        => &mut self.io,
            Screen    => &mut self.screen,
            Illegal   => panic!("Illegal! inspect from {:02x}", _addr),
        }
    }

}

impl MemoryIO for SimpleMem {
    fn upload(&mut self, _addr : u16, _data : &[u8]) {
        unimplemented!("TBD")
    }

    fn get_range(&self) -> (u16, u16) {
        (0, 0xffff)
    }

    fn update_sha1(&self, _digest : &mut Sha1) {
        unimplemented!("TBD")
    }

    fn inspect_byte(&self, addr:u16) -> u8 {
        let reg = self.get_region(addr);
        reg.inspect_byte(addr)
    }

    fn load_byte(&mut self, addr:u16) -> u8 {
        let reg = self.get_region_mut(addr);
        reg.load_byte(addr)
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        let reg = self.get_region_mut(addr);
        reg.store_byte(addr, val)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}


// use cpu::{Regs, StandardClock, Clock, InstructionDecoder};

use cpu::{Regs, StandardClock};
use cpu;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Simple {
    regs        : Regs,
    mem         : SimpleMem,
    gdb_enabled : bool,
    rc_clock    : Rc<RefCell<StandardClock>>,
}

use std::sync::mpsc;
use gdbstub;
use std::thread;
use std::net::{TcpListener};

impl Simple {
    pub fn reset(&mut self) {
        cpu::reset(&mut self.regs, &mut self.mem);
    }

    pub fn new() -> Self {

        let rc_clock = Rc::new(RefCell::new(StandardClock::new(2_000_000)));

        let mem = SimpleMem::new();
        let regs = Regs::new();
        let gdb_enabled = false;

        let mut ret = Simple {
            mem, regs, gdb_enabled, rc_clock
        };

        ret.reset();

        ret
    }

    pub fn from_matches(_matches : &ArgMatches) -> Self {

        let ret = Self::new();


        ret
    }

    pub fn run(&mut self) {
        let mut cpu = gdbstub::Cpu::new();
        let mut debugger = gdbstub::Debugger::new();

        let (tx, rx) = mpsc::channel();

        let listener = TcpListener::bind("127.0.0.1:6809").unwrap();

        thread::spawn(move || {
            let rem = gdbstub::GdbRemote::new(&listener);
            tx.send(rem).unwrap()
        });

        let mut gdb : Option<gdbstub::GdbRemote> = None;

        loop {
            if gdb.is_none() {
                let is_gdb = rx.try_recv();
                if !is_gdb.is_err() {
                    gdb = Some(is_gdb.unwrap());
                }
            }

            if gdb.is_none() {
                cpu::step(&mut self.regs, &mut self.mem, &self.rc_clock);

                if self.mem.io.get_halt() {
                    self.mem.io.clear_halt();
                }
            }

            if gdb.is_some() {

                if let Some(ref mut remote) = gdb {

                    let ret = remote.serve(&mut debugger, &mut cpu);

                    match ret {
                        Err(_) => (),
                        _ => (),
                    }
                }
            }
        }
    }
}

impl gdbstub::DebuggerHost for Simple {

    fn force_pc(&mut self, pc : u16) {
        self.regs.pc = pc;
    }

    fn read_registers(&self, reply : &mut gdbstub::reply::Reply) {

        let regs = &self.regs;

        let cc = regs.flags.bits();

        reply.push_u8(cc);
        reply.push_u8(regs.a);
        reply.push_u8(regs.b);
        reply.push_u8(regs.dp);

        reply.push_u16(regs.x);
        reply.push_u16(regs.y);
        reply.push_u16(regs.u);
        reply.push_u16(regs.s);
        reply.push_u16(regs.pc);
    }

    fn resume(&mut self) {
    }

    fn set_step(&mut self)  {
    }

    fn add_breakpoint(&mut self, _addr : u16) {
    }

    fn add_write_watchpoint (&mut self, _addr : u16) {
    }

    fn add_read_watchpoint(&mut self, _addr : u16) {
    }

    fn del_breakpoint(&mut self, _addr : u16) {
    }

    fn del_write_watchpoint(&mut self, _addr : u16) {
    }

    fn del_read_watchpoint(&mut self, _addr : u16) {
    }

    fn examine(&self, _addr : u16) -> u8 {
        self.mem.inspect_byte(_addr)
    }
}
