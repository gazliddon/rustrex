use std::net::{TcpListener, Shutdown,TcpStream};
use std::io::{Read, Write};
use gdbstub::reply::{Reply, Endian};

use gdbstub::sigs::*;

////////////////////////////////////////////////////////////////////////////////

pub trait DebuggerHost {
    fn do_break(&mut self);
    fn read_registers(&self, _reply : &mut Reply) ;
    fn write_registers(&mut self, _data : &[u8]) ;
    fn force_pc(&mut self, _pc : u16) ;
    fn resume(&mut self) ;
    fn set_step(&mut self) ;
    fn add_breakpoint(&mut self, _addr : u16) ;
    fn add_write_watchpoint (&mut self, _addr : u16);
    fn add_read_watchpoint(&mut self, _addr : u16);
    fn del_breakpoint(&mut self, _addr : u16) ;
    fn del_write_watchpoint(&mut self, _addr : u16) ;
    fn del_read_watchpoint(&mut self, _addr : u16) ;
    fn examine(&self, _addr : u16) -> u8 ;
    fn write(&mut self, _addr : u16, val : u8);
}

pub struct GdbRemote {
    remote: TcpStream,
    endian : Endian,
}

pub type GdbResult = Result<(), ()>;
fn args_as_string(data : &[u8]) -> String {
    String::from_utf8(data.to_vec()).unwrap()
}

impl GdbRemote {
    pub fn new(listener: &TcpListener) -> GdbRemote {

        info!("Debugger waiting for gdb connection...");

        let remote =
            match listener.accept() {
                Ok((stream, sockaddr)) => {
                    info!("Connection from {}", sockaddr);
                    stream
                }
                Err(e) => panic!("Accept failed: {}", e),
            };

        GdbRemote {
            remote,
            endian : Endian::Big
        }
    }

    // Serve a single remote request
    pub fn serve(&mut self,
                 host: &mut DebuggerHost) -> GdbResult {

        match self.next_packet() {

            PacketResult::Ok(packet) => {
                try!(self.ack());
                self.handle_packet(host,  &packet)
            }
            PacketResult::BadChecksum(_) => {
                // Request retransmission
                self.nack()
            }
            PacketResult::Break =>  {
                try!(self.ack());
                host.do_break();
                Ok(())
            }

            PacketResult::EndOfStream => {
                // Session over
                Err(())
            }

            PacketResult::NoPacket => {
                Ok(())
            }
        }
    }

    /// Attempt to return a single GDB packet.
    fn next_packet(&mut self) -> PacketResult {

        let mut buf = [0;1];
        info!("about to peek bytes");

        if let Ok(_) = self.remote.peek(&mut buf)  {
            if buf[0] == 0x03 {
                let _ = self.remote.read_exact(&mut buf);
                return PacketResult::Break;
            }
        } else {
            return PacketResult::NoPacket;
        }

        // Parser state machine
        enum State {
            WaitForStart,
            InPacket,
            WaitForCheckSum,
            WaitForCheckSum2(u8),
        };

        let mut state = State::WaitForStart;

        let mut packet = Vec::new();
        let mut csum = 0u8;
        info!("about to read bytes");

        for r in (&self.remote).bytes() {

            let byte =
                match r {
                    Ok(b)  => b,
                    Err(e) => {
                        warn!("GDB remote error: {}", e);
                        return PacketResult::EndOfStream;
                    }
                };

            match state {
                State::WaitForStart => {
                    if byte == b'$' {
                        // Start of packet
                        state = State::InPacket;
                    }
                }
                State::InPacket => {
                    if byte == b'#' {
                        // End of packet
                        state = State::WaitForCheckSum;
                    } else {
                        // Append byte to the packet
                        packet.push(byte);
                        // Update checksum
                        csum = csum.wrapping_add(byte);
                    }
                }
                State::WaitForCheckSum => {
                    match ascii_hex(byte) {
                        Some(b) => {
                            state = State::WaitForCheckSum2(b);
                        }
                        None => {
                            warn!("Got invalid GDB checksum char {}",
                                  byte);
                            return PacketResult::BadChecksum(packet);
                        }
                    }
                }
                State::WaitForCheckSum2(c1) => {
                    match ascii_hex(byte) {
                        Some(c2) => {
                            let expected = (c1 << 4) | c2;

                            if expected != csum {
                                warn!("Got invalid GDB checksum: {:x} {:x}",
                                      expected, csum);
                                return PacketResult::BadChecksum(packet);
                            }

                            // Checksum is good, we're done!
                            return PacketResult::Ok(packet);
                        }
                        None => {
                            warn!("Got invalid GDB checksum char {}",
                                  byte);
                            return PacketResult::BadChecksum(packet);
                        }
                    }
                }
            }
        }

        warn!("GDB remote end of stream");

        PacketResult::EndOfStream
    }

    /// Acknowledge packet reception
    fn ack(&mut self) -> GdbResult {
        if let Err(e) = self.remote.write(b"+") {
            warn!("Couldn't send ACK to GDB remote: {}", e);
            Err(())
        } else {
            Ok(())
        }
    }

    /// Request packet retransmission
    fn nack(&mut self) -> GdbResult {
        if let Err(e) = self.remote.write(b"-") {
            warn!("Couldn't send NACK to GDB remote: {}", e);
            Err(())
        } else {
            Ok(())
        }
    }

    fn disconnect(&mut self) -> GdbResult {
        try!(self.send_ok());
        self.remote.shutdown(Shutdown::Both).unwrap();
        Ok(())
    }

    fn handle_query(&mut self, args : &[u8]) -> GdbResult {
        use std::str;

        let text = str::from_utf8(args).unwrap();

        match text {
            "Attached" => self.send_string(b"1"),
            "TStatus" => self.send_string(b"T0"),
            "C" => self.send_empty_reply(),
            _ => {

                info!("unhandled q {}", text);
                self.send_empty_reply()
            }
        }
    }

    fn has_break(&mut self) {
    }

    fn handle_packet(&mut self,
                     host: &mut DebuggerHost,
                     packet: &[u8]) -> GdbResult {

        use std::str;

        let command = packet[0] as char;
        let args = &packet[1..];

        let packet_str = str::from_utf8(packet).unwrap();
        let args_str = str::from_utf8(args).unwrap();

        let to_check = "zZ";

        if to_check.find(command).is_some() {
            info!("raw cmd {}", packet_str);
        }

        let res = match command {
            'q' => self.handle_query(args),
            '?' => self.send_status(),
            'D' => self.disconnect(),
            'M' => self.write_memory(host, args),
            'm' => self.read_memory(host, args),
            'g' => self.read_registers(host),
            'G' => self.write_registers(host, args),
            'c' => self.resume(host, args),
            's' => self.step(host, args),

            'Z' => self.add_breakpoint(host, args),
            'z' => self.del_breakpoint(host, args),

            // Send empty response for unsupported packets
            _ => {
                info!("unhandled command {} {:?}", command, args_str);
                self.send_empty_reply()
            },
        };

        // Check for errors
        try!(res);

        Ok(())
    }

    fn send_reply(&mut self, reply: Reply) -> GdbResult {
        match self.remote.write(&reply.into_packet()) {
            // XXX Should we check the number of bytes written? What
            // do we do if it's less than we expected, retransmit?
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("Couldn't send data to GDB remote: {}", e);
                Err(())
            }
        }
    }

    fn send_empty_reply(&mut self) -> GdbResult {
        let reply = Reply::new(&self.endian);
        self.send_reply(reply)
    }

    fn send_string(&mut self, string: &[u8]) -> GdbResult {
        let mut reply = Reply::new(&self.endian);
        reply.push(string);
        self.send_reply(reply)
    }

    fn send_error(&mut self) -> GdbResult {
        // GDB remote doesn't specify what the error codes should
        // be. Should be bother coming up with our own convention?
        self.send_string(b"E00")
    }

    pub fn send_status(&mut self) -> GdbResult {
        // Maybe we should return different values depending on the
        // cause of the "break"?
        self.send_string(b"S00")
    }

    pub fn send_ok(&mut self) -> GdbResult {
        self.send_string(b"OK")
    }

    fn write_memory(&mut self, _host : &mut DebuggerHost, args: &[u8]) -> GdbResult {
        let (addr, data) = try!(parse_write_mem(args));

        let addr = addr as u16;

        for (i, v) in data.iter().enumerate() {
            let a = addr.wrapping_add(i as u16);
            _host.write(a, *v)
        }

        self.send_ok()
    }

    /// Read a region of memory. The packet format should be
    /// `ADDR,LEN`, both in hexadecimal
    fn read_memory(&mut self, host : &mut DebuggerHost, args: &[u8]) -> GdbResult {

        let mut reply = Reply::new(&self.endian);

        let (addr, len) = try!(parse_addr_len(args));

        if len == 0 {
            // Should we reply with an empty string here? Probably
            // doesn't matter
            return self.send_error();
        }

        for i in 0..len {
            let a = (addr as u16).wrapping_add(i as u16);
            let b = host.examine(a);
            reply.push_u8(b);
        }

        self.send_reply(reply)
    }

    /// Continue execution
    fn resume(&mut self,
              host: &mut DebuggerHost,
              args: &[u8]) -> GdbResult {

        if !args.is_empty() {
            // If an address is provided we restart from there
            let addr = try!(parse_hex(args));
            host.force_pc(addr as u16);
        }

        // Tell the debugger we want to resume execution.
        host.resume();
        Ok(())
    }

    fn read_registers(&mut self, host : & mut DebuggerHost) -> GdbResult {
        let mut reply = Reply::new(&self.endian);
        host.read_registers(&mut reply);
        self.send_reply(reply)
    }

    fn write_registers(&mut self, host : &mut DebuggerHost, args: &[u8]) -> GdbResult {

        let data = try!(parse_data(args));

        host.write_registers(&data);

        self.send_ok()
    }

    // Step works exactly like continue except that we're only
    // supposed to execute a single instruction.
    fn step(&mut self,
            host: &mut DebuggerHost,
            _args: &[u8]) -> GdbResult {

        host.set_step();

        self.send_trap()

            // self.resume(host, args)
    }


    fn send_sig(&mut self, v : Sigs) -> GdbResult {
        let v = format!("S{:02X}", v as u8);
        let bytes = v.into_bytes();
        self.send_string(&bytes)
    }

    pub fn send_trap(&mut self) -> GdbResult { self.send_sig(Sigs::SIGTRAP) }
    pub fn send_int(&mut self) -> GdbResult { self.send_sig(Sigs::SIGINT) }

    // Add a breakpoint or watchpoint
    fn add_breakpoint(&mut self,
                      host: &mut DebuggerHost,
                      args: &[u8]) -> GdbResult {

        info!("add_breakpoint {}", args_as_string(args));

        // Check if the request contains a command list
        if args.iter().any(|&b| b == b';') {
            // Not sure if I should signal an error or send an empty
            // reply here to signal that command lists are not
            // supported. I think GDB will think that we don't support
            // this breakpoint type *at all* if we return an empty
            // reply. I don't know how it handles errors however.
            return self.send_error();
        };

        let (btype, addr, kind) = try!(parse_breakpoint(args));

        // Only kind "4" makes sense for us: 32bits standard MIPS mode
        // breakpoint. The MIPS-specific kinds are defined here:
        // https://sourceware.org/gdb/onlinedocs/gdb/MIPS-Breakpoint-Kinds.html
        if kind != b'4' {
            // Same question as above, should I signal an error?
            return self.send_error();
        }

        match btype {
            b'0' => host.add_breakpoint(addr as u16),
            b'2' => host.add_write_watchpoint(addr as u16),
            b'3' => host.add_read_watchpoint(addr as u16),
            // Unsupported breakpoint type
            _ => return self.send_empty_reply(),
        }

        self.send_ok()
    }

    // Delete a breakpoint or watchpoint
    fn del_breakpoint(&mut self,
                      host: &mut DebuggerHost,
                      args: &[u8]) -> GdbResult {

        let (btype, addr_big, kind) = try!(parse_breakpoint(args));

        info!("del_breakpoint {}", args_as_string(args));

        let addr = addr_big as u16;

        // Only 32bits standard MIPS mode breakpoint supported
        if kind != b'4' {
            return self.send_error();
        }

        match btype {
            b'0' => host.del_breakpoint(addr),
            b'2' => host.del_write_watchpoint(addr),
            b'3' => host.del_read_watchpoint(addr),
            // Unsupported breakpoint type
            _ => return self.send_empty_reply(),
        }

        self.send_ok()
    }

}

enum PacketResult {
    Ok(Vec<u8>),
    BadChecksum(Vec<u8>),
    EndOfStream,
    Break,
    NoPacket,
}

/// Get the value of an integer encoded in single lowercase
/// hexadecimal ASCII digit. Return None if the character is not valid
/// hexadecimal
fn ascii_hex(b: u8) -> Option<u8> {
    if b >= b'0' && b <= b'9' {
        Some(b - b'0')
    } else if b >= b'a' && b <= b'f' {
        Some(10 + (b - b'a'))
    } else {
        // Invalid
        None
    }
}

fn ascii_hex_err(b: u8) -> Result<u8, ()> {
    if b >= b'0' && b <= b'9' {
        Ok(b - b'0')
    } else if b >= b'a' && b <= b'f' {
        Ok(10 + (b - b'a'))
    } else {
        // Invalid
        Err(())
    }
}


/// Parse an hexadecimal string and return the value as an
/// integer. Return `None` if the string is invalid.
fn parse_hex(hex: &[u8]) -> Result<u32, ()> {
    let mut v = 0u32;

    for &b in hex {
        v <<= 4;

        v |=
            match ascii_hex(b) {
                Some(h) => h as u32,
                // Bad hex
                None => return Err(()),
            };
    }

    Ok(v)
}

use itertools::Itertools;

/// Parse an hexadecimal string and return the value as an
/// integer. Return `None` if the string is invalid.

fn parse_data(_hex: &[u8]) -> Result<Vec<u8>, ()> {

    let mut res = vec!();

    for (l, h) in _hex.iter().tuples() {

        let hn = try!(ascii_hex_err(*l));
        let ln = try!(ascii_hex_err(*h));
        res.push(ln | hn << 4);
    }

    Ok(res)
}


/// Parse a string in the format `addr,len` (both as hexadecimal
/// strings) and return the values as a tuple. Returns `None` if
/// the format is bogus.
fn parse_addr_len(args: &[u8]) -> Result<(u32, u32), ()> {

    // split around the comma
    let args: Vec<_> = args.split(|&b| b == b',').collect();

    if args.len() != 2 {
        // Invalid number of arguments
        return Err(());
    }

    let addr = args[0];
    let len = args[1];

    if addr.is_empty() || len.is_empty() {
        // Missing parameter
        return Err(());
    }

    // Parse address
    let addr = try!(parse_hex(addr));
    let len = try!(parse_hex(len));

    Ok((addr, len))
}

fn parse_write_mem(args: &[u8]) -> Result<(u32, Vec<u8>), ()> {

    let args: Vec<_> = args.split(|&b| b == b',').collect();

    if args.len() != 2 { return Err(()) }

    let command: Vec<_> = args[1].split(|&b| b== b':').collect();

    if command.len() != 2 { return Err(()) }

    let addr = try!(parse_hex(args[0]));
    let len = try!(parse_hex(command[0]));
    let bytes = try!(parse_data(command[1]));

    if len as usize != bytes.len() {
        Err(())
    } else {
        Ok((addr, bytes))
    }
}

/// Parse breakpoint arguments: the format is
/// `type,addr,kind`. Returns the three parameters in a tuple or an
/// error if a format error has been encountered.

fn parse_breakpoint(args: &[u8]) -> Result<(u8, u32, u8), ()> {
    // split around the comma

    let args: Vec<_> = args.split(|&b| b == b',').collect();

    if args.len() != 3 {
        // Invalid number of arguments
        return Err(());
    }

    let btype = args[0];
    let addr = args[1];
    let kind = args[2];

    if btype.len() != 1 || kind.len() != 1 {
        // Type and kind should only be one character each
        return Err(());
    }

    let btype = btype[0];
    let kind = kind[0];

    let addr = try!(parse_hex(addr));

    Ok((btype, addr, kind))
}
