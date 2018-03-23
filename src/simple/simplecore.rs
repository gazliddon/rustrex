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
use cpu;
use gdbstub;

use clap::{ArgMatches};
use cpu::{Regs, StandardClock};

use mem::*;

use simple::Io;
use simple::GdbConnection;

use window;

use std::rc::Rc;
use std::cell::RefCell;

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

fn pix_to_rgb(p : u8, palette : &[u8], dest : &mut[u8])  {
    let p = p as usize;
    let palette = &palette[p * 3 ..];
    dest.copy_from_slice(&palette[..3]);
}

fn to_rgb(mem : &[u8], palette : &[u8]) -> [u8; 304 * 256 * 3]{

    let mut ret : [u8; 304 * 256 * 3] = [0; 304 * 256 * 3];

    for (i, b) in mem.iter().enumerate() {

        let x = (i / 256) * 2;
        let y = i & 0xff;
        let d = x + y * 304 * 3;
        let dest = &mut ret[d..];

        pix_to_rgb(b&0xff, palette, &mut dest[..3]);
        pix_to_rgb(b>>4, palette, &mut dest[..3]);
    };

    ret
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


pub struct Simple {
    regs        : Regs,
    mem         : SimpleMem,
    rc_clock    : Rc<RefCell<StandardClock>>,
}

impl gdbstub::DebuggerHost for Simple {

    fn force_pc(&mut self, pc : u16) {
        self.regs.pc = pc;
    }

    fn read_registers(&self, reply : &mut gdbstub::Reply) {

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
        warn!("unimplemented resume")
    }

    fn set_step(&mut self)  {
        cpu::step(&mut self.regs, &mut self.mem, &self.rc_clock);
    }

    fn add_breakpoint(&mut self, _addr : u16) {
        warn!("unimplemented add_breakpoint")
    }

    fn add_write_watchpoint (&mut self, _addr : u16) {
        warn!("unimplemented add_write_watchpoint")
    }

    fn add_read_watchpoint(&mut self, _addr : u16) {
        warn!("unimplemented add_read_watchpoint")
    }

    fn del_breakpoint(&mut self, _addr : u16) {
        warn!("unimplemented del_breakpoint")
    }

    fn del_write_watchpoint(&mut self, _addr : u16) {
        warn!("unimplemented del_write_watchpoint")
    }

    fn del_read_watchpoint(&mut self, _addr : u16) {
        warn!("unimplemented del_read_watchpoint")
    }

    fn examine(&self, _addr : u16) -> u8 {
        self.mem.inspect_byte(_addr)
    }

    fn write (&mut self, addr : u16, val : u8) {
        self.mem.store_byte(addr, val)
    }

    fn write_registers(&mut self, _data : &[u8]) {
        let it = &mut _data.iter();

        let cc = pop_u8(it);
        self.regs.flags.set_flags(cc);

        self.regs.a  = pop_u8(it);
        self.regs.b  = pop_u8(it);
        self.regs.dp = pop_u8(it);

        self.regs.x  = pop_u16(it);
        self.regs.y  = pop_u16(it);
        self.regs.u  = pop_u16(it);
        self.regs.s  = pop_u16(it);
        self.regs.pc = pop_u16(it);
    }
}

fn pop_u8<'a, I>(vals: &mut I) -> u8
where
    I: Iterator<Item = &'a u8>,
{
    *vals.next().unwrap()
}

fn pop_u16<'a, I>(vals: &mut I) -> u16
where
    I: Iterator<Item = &'a u8>,
{
    let h = *vals.next().unwrap() as u16;
    let l = *vals.next().unwrap() as u16;

    l | (h << 8)
}


impl Simple {

    pub fn reset(&mut self) {
        cpu::reset(&mut self.regs, &mut self.mem);
    }

    pub fn new() -> Self {

        let rc_clock = Rc::new(RefCell::new(StandardClock::new(2_000_000)));

        let mem = SimpleMem::new();
        let regs = Regs::new();

        let mut ret = Simple {
            mem, regs, rc_clock
        };

        ret.reset();

        ret
    }

    pub fn from_matches(_matches : &ArgMatches) -> Self {
        let ret = Self::new();
        ret
    }

    pub fn run(&mut self) {
        let buffer = {
            let scr = &self.mem.screen.data;
            let pal = &self.mem.io.palette;
            to_rgb(scr, pal)
        };

        let w = &mut window::Window::new("my lovely window");

        let mut conn = GdbConnection::new();

        window::run_loop(|| {
            conn.update(self);
            w.update()
        });
    }
}


