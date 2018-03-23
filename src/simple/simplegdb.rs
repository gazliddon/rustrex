use std::sync::mpsc;
use std::thread;

use std::net::{TcpListener};

use gdbstub;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnState {
    Start,
    Waiting,
    Connected,
}

pub struct GdbConnection {
    state : ConnState,
    gdb : Option<gdbstub::GdbRemote>,
    tx : mpsc::Sender<gdbstub::GdbRemote>,
    rx : mpsc::Receiver<gdbstub::GdbRemote>,
}

impl GdbConnection {

    pub fn new() -> Self {
        use self::ConnState::*;

        let state = Start;
        let gdb = None;
        let (tx, rx) = mpsc::channel();

        Self {
            state, gdb, tx, rx
        }
    }

    pub fn update(&mut self, host : &mut gdbstub::DebuggerHost) -> ConnState {

        use self::ConnState::*;

        let state = self.state.clone();

        match state {

            Start => {
                self.state = Waiting;

                let tx = self.tx.clone();

                thread::spawn(move || {
                    let listener = TcpListener::bind("127.0.0.1:6809").unwrap();
                    let rem = gdbstub::GdbRemote::new(&listener);
                    tx.send(rem).unwrap();
                });

                info!("Waiting for gdb connection")
            },

            Waiting => {
                let is_gdb = self.rx.try_recv();

                if !is_gdb.is_err() {
                    self.state = Connected;
                    self.gdb = Some(is_gdb.unwrap());
                    info!("gdb connected")
                }
            },

            Connected => {
                let mut ret = Err(());

                if let Some(ref mut remote) = self.gdb {
                    ret = remote.serve(host);
                }

                match ret {
                    Err(_) => { 
                        info!("gdb disconnected");
                        self.state = Start;
                    },
                    _ => (),
                }
            }

        }

        state
    }
}

