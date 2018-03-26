use std::net::{TcpListener, };
use std::sync::mpsc;
use std::thread;

use gdbstub::{ DebuggerHost, GdbRemote, Reply };

pub enum Events {
    Disconnected,
    Connected,
    DoBreak,
    ForcePc(u16),
    ReadRegisters,
    Resume,
    Step,
    WriteRegisters,
    Examine(u16),
    Write(u16, u8),
}

struct DebuggerProxy {
    pub tx  : mpsc::Sender<Events>,
}

impl DebuggerProxy {
    fn  send(&self, ev : Events) {
        let _ = self.tx.send(ev);
    }
}

impl DebuggerHost for DebuggerProxy {

    fn do_break(&mut self) {
        self.send(Events::DoBreak)
    }

    fn read_registers(&self, _reply : &mut Reply)  {
        self.send(Events::ReadRegisters)
    }

    fn write_registers(&mut self, _data : &[u8])  {
        self.send(Events::WriteRegisters)
    }

    fn examine(&self, addr : u16) -> u8 {
        self.send(Events::Examine(addr));
        0
    }

    fn write(&mut self, addr : u16, val : u8) {
        self.send(Events::Write(addr, val))
    }

    fn resume(&mut self) {
        self.send(Events::Resume)
    }

    fn force_pc(&mut self, _pc : u16)  {
        self.send(Events::ForcePc(_pc))
    }

    fn set_step(&mut self) {
        self.send(Events::Step)
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
}



struct ThreadedGdb {
    pub rx  : mpsc::Receiver<Events>,
}

impl ThreadedGdb {

    pub fn new() -> ThreadedGdb {
        let (tx, rx) =  mpsc::channel();

        thread::spawn(move || {

            let listener = TcpListener::bind("127.0.0.1:6809").unwrap();
            let mut remote = GdbRemote::new(&listener);

            let _ = tx.send(Events::Connected);

            let mut dbg_proxy = DebuggerProxy { tx : tx.clone() };

            loop {
                let ret  = remote.serve(&mut dbg_proxy);

                match ret {
                    Err(_) => break,
                    _ => ()
                };
            }

            let _ = tx.send(Events::Disconnected);
        });

        ThreadedGdb {
            rx
        }
    }
}

