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

use clap::{ArgMatches};
use cpu::{Regs, StandardClock};

use mem::*;

use simple::Io;

use window;

use std::rc::Rc;
use std::cell::RefCell;

use gdbstub::{ ThreadedGdb, Message, Sigs};
use gdbstub;

use utils;
use state;

use breakpoints::{BreakPoint, BreakPoints, BreakPointTypes};

#[derive(Debug, Clone, Copy, PartialEq)]
enum SimState {
    Paused,
    Running,
    Quitting,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimEvent {
    Debugger(Message),
    Halt(Sigs),
    HitSync,
    Pause,
    Quit,
    RomChanged,
    MaxCycles,
    Reset,
}
// Extend breakpoint to be initialisable from gdb bp descriptions

impl BreakPoint {

    pub fn from_gdb_type( bp_type : gdbstub::BreakPointTypes, addr : u16 ) -> Self {
        use gdbstub::BreakPointTypes::*;

        let my_type = match bp_type {
            Read  => BreakPointTypes::READ,
            Write => BreakPointTypes::WRITE,
            Exec  => BreakPointTypes::EXEC,
        };

        Self::new(my_type, addr)
    }
}

////////////////////////////////////////////////////////////////////////////////


use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::{ channel, Receiver };
use std::time::Duration;

const W : usize = 304;
const H : usize = 256;
const DIMS : (u32, u32) = (W as u32, H as u32);
const SCR_BYTES : usize = W * H * 3; 

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

fn to_rgb(mem : &[u8], palette : &[u8]) -> [u8; SCR_BYTES]{
    let mut ret : [u8; SCR_BYTES] = [0; SCR_BYTES];

    for (i, b) in mem.iter().enumerate() {

        let x = (i / H) * 2;
        let y = i % H;
        let d = ( x + y * W )  * 3;

        let dest = &mut ret[d..];

        pix_to_rgb(b&0xf, palette, &mut dest[..3]);
        pix_to_rgb(b>>4, palette, &mut dest[3..6]);
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

pub struct Simple {
    regs         : Regs,
    mem          : SimpleMem,
    rc_clock     : Rc<RefCell<StandardClock>>,
    file         : Option<String>,
    watcher      : Option<FileWatcher>,
    events       : Vec<SimEvent>,
    win          : window::Window,
    dirty        : bool,
    gdb          : ThreadedGdb,
    break_points : BreakPoints,
}

impl Simple {
    pub fn new() -> Self {
        let rc_clock = Rc::new(RefCell::new(StandardClock::new(2_000_000)));

        let mem = SimpleMem::new();
        let regs = Regs::new();
        let win = window::Window::new("my lovely window", DIMS);

        let gdb = ThreadedGdb::new();

        let break_points = BreakPoints::new();

        let ret = Simple {
            mem, regs, rc_clock, win, gdb, break_points,
            file    : None,
            watcher : None,
            events  : vec![],
            dirty   : false,
        };

        ret
    }

    pub fn step(&mut self) -> Option<SimEvent> {
        let res = cpu::step(&mut self.regs, &mut self.mem, &self.rc_clock);

        let ret =  match res {
            Ok(i) => {
                if i.op_code == 0x13 {
                    Some(SimEvent::HitSync)
                } else {
                    None
                }
            }
            Err(_cpu_err) => {
                Some(SimEvent::Halt(Sigs::SIGILL))
            }
        };

        if let Some(ref ev) = ret {
            self.add_event(ev.clone());
        };

        ret
    }

    pub fn reset(&mut self) {
        cpu::reset(&mut self.regs, &mut self.mem);
        info!("Reset! pc=${:03x}", self.regs.pc);
    }

    fn handle_file_watcher(&mut self)  {
        let mut has_changed = false;

        if let Some(ref mut watcher) = self.watcher {
            if watcher.has_changed() {
                has_changed = true;
            }
        }

        if has_changed {
            self.add_event(SimEvent::RomChanged);
        }
    }

    fn rom_changed(&mut self) {
        self.load_rom();
        self.reset();
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

    fn run_to_sync(&mut self, max_instructions : usize ) -> Option<SimEvent> {
        // run for n instructions OR
        // stop on an event
        // Could be an error or whatever

        for _ in 0..max_instructions {

            let ret = self.step();

            if ret.is_some() {
                return ret;
            }
        }
        None
    }

    fn add_event(&mut self, event : SimEvent) {
        self.events.push(event)
    }

    pub fn handle_window(&mut self) {
        use window::Action;

        for ev in self.win.update() {
            let sim_event = match ev {
                Action::Reset    => Some(SimEvent::Reset),
                Action::Quit     => Some(SimEvent::Quit),
                Action::Pause    => Some(SimEvent::Pause),
                Action::Continue => None
            };
            if let Some(event) = sim_event {
                self.add_event(event);
            }
        }
    }

    pub fn handle_debugger(&mut self) {


        loop {
            if let Some(msg) = self.gdb.poll() {
                match msg {
                    Message::Disconnected | Message::Connected | Message::DoBreak  => {
                        self.add_event(SimEvent::Debugger(msg));
                        self.gdb.ack()
                    },

                    Message::Step => {
                        warn!("Should send back correct sig for step mode");
                        self.add_event(SimEvent::Debugger(msg));
                        self.gdb.ack()
                    }

                    Message::SetReg(register_number, v16) => {

                        let regs = &mut self.regs;

                        let v8 = v16 as u8;

                        match register_number {
                            0 => { regs.flags.set_flags(v8) }
                            1 => { regs.a = v8; }
                            2 => { regs.b = v8; }
                            3 => { regs.dp = v8;}
                            4 => { regs.x = v16 }
                            5 => { regs.y = v16 }
                            6 => { regs.s = v16 }
                            7 => { regs.u = v16 }
                            8 => { regs.pc = v16}
                            _ => { warn!("illgal reg num {} for set_reg", register_number) }
                        };

                        self.gdb.ack()
                    },

                    Message::GetReg(register_number) => {

                        let regs = &mut self.regs;

                        let val = match register_number {
                            0 => { regs.flags.bits() as u16}
                            1 => { regs.a as u16}
                            2 => { regs.b as u16}
                            3 => { regs.dp as u16}
                            4 => { regs.x }
                            5 => { regs.y }
                            6 => { regs.s }
                            7 => { regs.u }
                            8 => { regs.pc }
                            _ => { warn!("illgal reg num {} for get_reg", register_number ); 0 }
                        };

                        self.gdb.reply(Message::SetReg(register_number, val))
                    },

                    Message::Resume => {
                        self.add_event(SimEvent::Debugger(msg))
                    }

                    Message::BreakPoint(bp_type, addr) => {
                        let break_point = BreakPoint::from_gdb_type(bp_type, addr);
                        self.break_points.add(&break_point);
                        self.gdb.ack()
                    }

                    Message::DeleteBreakPoint(bp_type, addr) => {
                        let break_point = BreakPoint::from_gdb_type(bp_type, addr);
                        self.break_points.remove(&break_point);
                        self.gdb.ack()
                    }

                    Message::ForcePc(addr) => {
                        self.regs.pc = addr;
                        self.gdb.ack();
                    }

                    Message::Examine(addr) => {
                        let reply =  Message::Write( addr, self.mem.inspect_byte(addr));
                        self.gdb.reply(reply);
                    }

                    Message::WriteRegisters(data) => {

                        let mut _it = data.iter();

                        macro_rules! take8 {
                            () => { _it.next().unwrap().clone() }
                        }

                        macro_rules! take16 {
                            () => ({
                                let h = take8!() as u16;
                                let l = take8!() as u16;
                                h << 8 | l
                            })
                        }

                        let regs = &mut self.regs;

                        regs.flags.set_flags(take8!());
                        regs.a = take8!();
                        regs.b = take8!();
                        regs.dp = take8!();
                        regs.x = take16!();
                        regs.y = take16!();
                        regs.s = take16!();
                        regs.u = take16!();
                        regs.pc = take16!();

                        info!("received registers and pc = ${:04x}", regs.pc);

                        self.gdb.ack();
                    }

                    Message::ReadRegisters => {
                        let regs = &self.regs;

                        let cc = regs.flags.bits();

                        let ret : Vec<u8> = vec![
                            cc, regs.a, regs.b, regs.dp,

                            (regs.x >> 8) as u8,
                            regs.x as u8,

                            (regs.y >> 8) as u8,
                            regs.y as u8,

                            (regs.u >> 8) as u8,
                            regs.u as u8,

                            (regs.s >> 8) as u8,
                            regs.s as u8,

                            (regs.pc >> 8) as u8,
                            regs.pc as u8,
                        ];

                        self.gdb.reply(Message::WriteRegisters(ret));
                    }

                    _ => info!("unimplemented msg {:?}", msg),
                }
            } else {
                break
            }
        }
    }

    pub fn update_texture(&mut self) {
        let buffer = {
            let scr = &self.mem.screen.data;
            let pal = &self.mem.io.palette;
            to_rgb(scr, pal)
        };

        self.win.update_texture(&buffer);
    }

    pub fn run(&mut self) {
        use self::SimEvent::*;
        let mut state = state::State::new(&SimState::Paused);

        self.reset();

        loop {
            self.handle_window();
            self.handle_file_watcher();
            self.handle_debugger();

            while let Some(event) = self.events.pop() {
                match event {
                    RomChanged => self.rom_changed(),
                    HitSync =>  self.update_texture(),
                    _ => (),
                };

                match state.get() {
                    SimState::Running => {
                        match event {
                            Pause => state.set(&SimState::Paused),
                            Quit => state.set(&SimState::Quitting),

                            Halt(sig) => {
                                self.gdb.reply(Message::Halt(sig));
                                state.set(&SimState::Paused)
                            }

                            Debugger(msg) => {
                                match msg {
                                    Message::Connected => state.set(&SimState::Paused),
                                    Message::DoBreak => state.set(&SimState::Paused),
                                    _ => warn!("Unhandled debugger msg {:?} in state {:?}", msg, state.get())
                                }
                            }
                            _ => (),
                        }
                    },

                    SimState::Paused => {
                        match event {
                            Pause => state.set(&SimState::Running),
                            Quit => state.set(&SimState::Quitting),

                            Debugger(msg) => {
                                match msg {
                                    Message::Resume => state.set(&SimState::Running),
                                    Message::Step => {self.step(); ()}
                                    Message::Disconnected => state.set(&SimState::Running),
                                    _ => warn!("Unhandled debugger msg {:?} in state {:?}", msg, state.get())
                                }
                            }
                            _ => ()
                        }
                    },

                    SimState::Quitting => {
                    },
                };
            };

            if state.has_changed() {
                info!("State changed: {:?}", state);
                state.clear_change();
            }

            match state.get() {
                SimState::Quitting => {
                    break;
                },

                SimState::Running => {
                    self.run_to_sync(2_000_000 / 60);
                    self.win.draw();
                }

                SimState::Paused => {
                    use std::{thread, time};
                    let sleep_time = time::Duration::from_millis(1);
                    thread::sleep(sleep_time);
                }
            };
        }
    }
}





