use crate::mem::{MemMap, MemoryIO};
use std::cell::RefCell;
use std::fmt;
use sha1::Sha1;

#[derive(Debug, Clone, Default)]

pub struct LogEntry {
    pub addr : u16,
    pub write : bool,
    pub val : u16,
    pub word : bool,
}

impl LogEntry {
    fn write_byte(addr : u16, val : u8) -> LogEntry {
        LogEntry {
            addr: addr,
            write: true,
            val : val as u16,
            word : false,
        }
    }

    fn read_byte(addr : u16, val : u8) -> LogEntry {
        LogEntry {
            addr: addr,
            write: false,
            val : val as u16,
            word : false,
        }
    }

    fn write_word(addr : u16, val : u16) -> LogEntry {
        LogEntry {
            addr: addr,
            write: true,
            val : val,
            word : true,
        }

    }

    fn read_word(addr : u16, val : u16) -> LogEntry {
        LogEntry {
            addr: addr,
            write: false,
            val : val,
            word : true,
        }

    }

}

impl fmt::Display for LogEntry {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let width_str = if self.word {
            "16"
        } else {
            "8 "
        };

        let (op_str, arr_str) = if self.write {
            ("W", "->")
        } else {
            ("R", "<-")
        };


        let val_str = if self.word {
            format!("{:04x}", self.val)
        } else {
            format!("  {:02x}", self.val)
        };

        write!(f, "{}{} {} {} {:04x}", op_str, width_str,val_str, arr_str, self.addr)
    }

}


pub struct LoggingMemMap {
    max_log_size : usize,
    mem_map: MemMap,
    log_cell : RefCell<Vec<LogEntry>>,
}

impl LoggingMemMap {

    pub fn new(mm : MemMap) -> LoggingMemMap {
        LoggingMemMap {
            max_log_size : 100,
            mem_map : mm,
            log_cell : RefCell::new(vec![]),
        }
    }

    pub fn get_log(&self) -> Vec<LogEntry> {
        self.log_cell.borrow().clone()
    }

    fn log(&self, txt : LogEntry) {
        let mut v = self.log_cell.borrow_mut();

        v.push(txt);

        if v.len() > self.max_log_size {
            v.truncate(self.max_log_size)
        }
    }

    pub fn clear_log(&mut self) {
        let mut v = self.log_cell.borrow_mut();
        v.truncate(0);
    }

}

impl MemoryIO for LoggingMemMap {
    fn update_sha1(&self, digest : &mut Sha1) {
        self.mem_map.update_sha1(digest)
    }

    fn upload(&mut self, addr : u16, data : &[u8]) {
        self.mem_map.upload(addr,data);
    }

    fn get_name(&self) -> String {
        self.mem_map.get_name()
    }

    fn get_range(&self) -> (u16, u16) {
        self.mem_map.get_range()
    }

    fn load_byte(&mut self, addr:u16) -> u8 {
        let val = self.mem_map.load_byte(addr);
        let msg = LogEntry::read_byte(addr, val);
        self.log(msg);
        val
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        self.mem_map.store_byte(addr, val);
        let msg = LogEntry::write_byte(addr, val);
        self.log(msg);
    }

    fn store_word(&mut self, addr:u16, val:u16) {
        self.mem_map.store_word(addr,val);
        let msg = LogEntry::write_word(addr, val);
        self.log(msg);
    }

    fn load_word(&mut self, addr:u16) -> u16 {
        let val = self.mem_map.load_word(addr);

        let msg = LogEntry::read_word(addr, val);
        self.log(msg);
        val
    }
}


