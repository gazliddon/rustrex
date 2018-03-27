use std::net::{TcpListener, };
use std::sync::mpsc;
use std::thread;

use gdbstub::{ DebuggerHost, GdbRemote, Reply };

pub enum Message {
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
    Ack,
}

struct DebuggerProxy {
    pub tx  : mpsc::Sender<Message>,
    pub rx  : mpsc::Receiver<Message>,
}

impl DebuggerProxy {

    pub fn send(&self, ev : Message) -> Message {
        let _ = self.tx.send(ev);
        unimplemented!()
    }

    pub fn ack(&self, _ev : Message) {
        unimplemented!("ack");
    }
}

impl DebuggerHost for DebuggerProxy {

    fn do_break(&mut self) {
        let _ret = self.send(Message::DoBreak);
    }

    fn read_registers(&self, _reply : &mut Reply)  {
        let _ret = self.send(Message::ReadRegisters);
    }

    fn write_registers(&mut self, _data : &[u8])  {
        let _ret = self.send(Message::WriteRegisters);
    }

    fn examine(&self, addr : u16) -> u8 {
        let _ret = self.send(Message::Examine(addr));
        0
    }

    fn write(&mut self, addr : u16, val : u8) {
        let _ret = self.send(Message::Write(addr, val));
    }

    fn resume(&mut self) {
        let _ret = self.send(Message::Resume);
    }

    fn force_pc(&mut self, _pc : u16)  {
        let _ret = self.send(Message::ForcePc(_pc));
    }

    fn set_step(&mut self) {
        let _ret = self.send(Message::Step);
    }

    fn add_breakpoint(&mut self, _addr : u16) {
        unimplemented!("add_breakpoint");
    }

    fn add_write_watchpoint (&mut self, _addr : u16) {
        unimplemented!("add_write_watchpoint ");
    }

    fn add_read_watchpoint(&mut self, _addr : u16) {
        unimplemented!("add_read_watchpoint");
    }

    fn del_breakpoint(&mut self, _addr : u16) {
        unimplemented!("del_breakpoint");
    }

    fn del_write_watchpoint(&mut self, _addr : u16) {
        unimplemented!("del_write_watchpoint");
    }

    fn del_read_watchpoint(&mut self, _addr : u16) {
        unimplemented!("del_read_watchpoint");
    }
}

struct ThreadedGdb {
    pub rx  : mpsc::Receiver<Message>,
    pub tx  : mpsc::Sender<Message>,
}

impl ThreadedGdb {

    pub fn new() -> ThreadedGdb {

        let (tx, rx) =  mpsc::channel();
        let (tx_client, rx_client) =  mpsc::channel();

        thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:6809").unwrap();
            let mut remote = GdbRemote::new(&listener);

            let _ = tx.send(Message::Connected);

            let mut dbg_proxy = DebuggerProxy { tx : tx.clone(), rx : rx_client, };

            loop {
                let ret  = remote.serve(&mut dbg_proxy);
                match ret {
                    Err(_) => break,
                    _ => ()
                };
            }

            let _ = tx.send(Message::Disconnected);
        });

        ThreadedGdb {
            rx,
            tx : tx_client
        }
    }

    pub fn reply(&mut self, _msg : Message) {
        unimplemented!();
    }

    pub fn poll(&mut self) -> Option<Message> {
        unimplemented!();
    }
}

