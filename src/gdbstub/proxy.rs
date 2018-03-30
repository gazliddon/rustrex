use std::net::{TcpListener, };
use std::sync::mpsc;
use std::thread;

use gdbstub::{ DebuggerHost, GdbRemote, Reply };

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Disconnected,
    Connected,
    DoBreak,
    ForcePc(u16),
    ReadRegisters,
    Resume,
    Step,
    WriteRegisters(Vec<u8>),
    Examine(u16),
    Write(u16, u8),
    Ack,
}

struct DebuggerProxy {
    pub tx  : mpsc::Sender<Message>,
    pub rx  : mpsc::Receiver<Message>,
}

impl DebuggerProxy {
    pub fn new(tx : mpsc::Sender<Message>, rx : mpsc::Receiver<Message>) -> Self {
        Self { tx , rx } }

    pub fn send(&self, ev : Message) -> Message {
        let _ = self.tx.send(ev);
        if let Ok(msg) = self.rx.recv() {
            msg
        } else {
            panic!("msg system fucked")
        }
    }

    pub fn send_wait_ack(&self, _ev : Message) {
        let msg = self.send(_ev);
        assert!(msg == Message::Ack);
    }

    pub fn ack(&self, _ev : Message) {
        let _ret = self.tx.send(Message::Ack);
    }
}

impl DebuggerHost for DebuggerProxy {
    fn do_break(&mut self) {
        self.send_wait_ack(Message::DoBreak);
    }

    fn read_registers(&self, reply : &mut Reply)  {
        if let Message::WriteRegisters(data) = self.send(Message::ReadRegisters) {
            for i in data {
                reply.push_u8(i)
            }
        } else {
            panic!("kjsakjska")
        }
    }

    fn write_registers(&mut self, data : &[u8])  {
        self.send_wait_ack(Message::WriteRegisters(data.to_vec()));
    }

    fn examine(&self, addr : u16) -> u8 {
        if let Message::Write(addr2, val) = self.send(Message::Examine(addr)) {
            assert!(addr2 == addr);
            val
        } else {
            panic!("kjsakjska")
        }
    }

    fn write(&mut self, addr : u16, val : u8) {
        self.send_wait_ack(Message::Write(addr, val));
    }

    fn resume(&mut self) {
        self.send_wait_ack(Message::Resume);
    }

    fn force_pc(&mut self, _pc : u16)  {
        self.send_wait_ack(Message::ForcePc(_pc));
    }

    fn set_step(&mut self) {
        self.send_wait_ack(Message::Step);
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

pub struct ThreadedGdb {
    pub rx  : mpsc::Receiver<Message>,
    pub tx  : mpsc::Sender<Message>,
}

impl ThreadedGdb {
    pub fn new() -> ThreadedGdb {
        let (tx, rx) =  mpsc::channel();
        let (tx_client, rx_client) =  mpsc::channel();

        thread::spawn(move || {
            let mut dbg_proxy = DebuggerProxy::new(tx, rx_client);

            let listener = TcpListener::bind("127.0.0.1:6809").unwrap();

            loop {
                let mut remote = GdbRemote::new(&listener);

                dbg_proxy.send_wait_ack(Message::Connected);

                loop {
                    let ret  = remote.serve(&mut dbg_proxy);
                    match ret {
                        Err(_) => break,
                        _ => ()
                    };
                }

                dbg_proxy.send_wait_ack(Message::Disconnected);
            }
        });

        ThreadedGdb {
            rx,
            tx : tx_client
        }
    }

    pub fn reply(&mut self, msg : Message) {
        self.tx.send(msg).unwrap()
    }

    pub fn ack(&mut self) {
        self.reply(Message::Ack)
    }

    pub fn poll(&mut self) -> Option<Message> {
        let val = self.rx.try_recv();
        match val {
            Ok(message )=> Some(message),
            Err(_) => None,
        }
    }
}

