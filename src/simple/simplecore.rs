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

use utils;

////////////////////////////////////////////////////////////////////////////////


use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::{ channel, Receiver };
use std::time::Duration;

struct FileWatcher {
    file : String,
    watcher : RecommendedWatcher,
    rx : Receiver<DebouncedEvent>,
}

impl FileWatcher {
    pub fn new(file : &str) -> Self {
        let (tx, rx)  = channel();

        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
        watcher.watch(file, RecursiveMode::Recursive).unwrap();

        Self { file : file.to_string(), watcher, rx }

    }
    pub fn has_changed(&mut self) -> bool {
        let msg = self.rx.try_recv();

        if !msg.is_err() {
            true
        } else {
            false
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
enum MemRegion {
    Illegal,
    Ram,
    IO,
    Screen,
}

struct SimpleMem {
    pub ram            : MemBlock,
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

        pix_to_rgb(b&0xf, palette, &mut dest[..3]);
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

    fn upload(&mut self, addr : u16, _data : &[u8]) {
        let mut addr = addr;

        for i in _data {
            self.store_byte(addr, *i);
            addr = addr.wrapping_add(1);
        }
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

    fn set_step(&mut self) {
        self.step();
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


pub struct Simple {
    regs       : Regs,
    mem        : SimpleMem,
    rc_clock   : Rc<RefCell<StandardClock>>,
    sync       : bool,
    file       : Option<String>,
    watcher    : Option<FileWatcher>,
}

impl Simple {
    pub fn new() -> Self {
        let rc_clock = Rc::new(RefCell::new(StandardClock::new(2_000_000)));

        let mem = SimpleMem::new();
        let regs = Regs::new();

        let ret = Simple {
            mem, regs, rc_clock,
            sync    : false,
            file    : None,
            watcher : None,
        };

        ret
    }

    pub fn step(&mut self) -> cpu::InstructionDecoder {
        let i = cpu::step(&mut self.regs, &mut self.mem, &self.rc_clock);

        if i.op_code == 0x13 {
            self.sync = true;
        }
        i 
    }

    pub fn reset(&mut self) {
        cpu::reset(&mut self.regs, &mut self.mem);
        info!("Reset! pc=${:03x}", self.regs.pc);
    }

    fn has_changed(&mut self) -> bool {
        if let Some(ref mut watcher) = self.watcher {
            watcher.has_changed()
        } else {
            false
        }
    }

    fn process_watches(&mut self) {
        if self.has_changed() {
            info!("File changed - reloading and resetting");
            self.load_rom();
            self.reset();
        }
    }

    fn load_rom(&mut self) {
        if let Some(ref file) = self.file {
            let data = utils::load_file(&file);
            self.mem.upload(0x9900, &data);
            info!("Loaded ROM: {}", file);
        }
    }

    pub fn from_matches(matches : &ArgMatches) -> Self {
        let mut ret = Self::new();
        let file = matches.value_of("ROM FILE").unwrap();
        ret.file = Some(file.to_string());
        ret.load_rom();
        ret.reset();

        if matches.is_present("watch-rom") {
            info!("Adding watch for rom file");

            let watcher = FileWatcher::new(file);
            ret.watcher = Some(watcher);
        }

        ret
    }

    fn run_to_halt(&mut self, max_instructions : usize ) {

        for _ in 0..max_instructions {
            self.step();

            if self.sync {
                break;
            }
        }

        self.sync = true;
    }


    pub fn run(&mut self) {

        let w = &mut window::Window::new("my lovely window");

        let mut conn = GdbConnection::new();

        window::run_loop(|| {

            self.process_watches();

            while self.sync == false {
                use simple::ConnState::*;

                let state = conn.update(self);

                match state {
                    Start | Waiting => {
                        self.run_to_halt(2_000_000);
                    },

                    Connected => (),
                }
            };

            self.sync = false;

            let buffer = {
                let scr = &self.mem.screen.data;
                let pal = &self.mem.io.palette;
                to_rgb(scr, pal)
            };

            w.update_texture(&buffer);
            let action = w.update();

            if action == window::Action::Reset {
                info!("Resetting");
                self.reset();
            }

            return action;

        });
    }
}


