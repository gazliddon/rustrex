use mem::{MemMap, MemoryIO};
use std::cell::RefCell;

pub struct LoggingMemMap {
    max_log_size : usize,
    mem_map: MemMap,
    log_cell : RefCell<Vec<String>>,
}

impl LoggingMemMap {

    pub fn new(mm : MemMap) -> LoggingMemMap {
        LoggingMemMap {
            max_log_size : 100,
            mem_map : mm,
            log_cell : RefCell::new(vec![]),
        }
    }

    pub fn get_log(&self) -> Vec<String> {
        self.log_cell.borrow().clone()
    }

    fn log(&self, txt : String) {
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

    fn upload(&mut self, addr : u16, data : &[u8]) {
        self.mem_map.upload(addr,data);
    }

    fn get_name(&self) -> String {
        self.mem_map.get_name()
    }

    fn get_range(&self) -> (u16, u16) {
        self.mem_map.get_range()
    }

    fn load_byte(&self, addr:u16) -> u8 {
        let val = self.mem_map.load_byte(addr);
        let msg = format!("R8          {:02x} -> {:02x}", addr, val);
        self.log(msg);
        val
    }

    fn store_byte(&mut self, addr:u16, val:u8) {
        self.mem_map.store_byte(addr, val);
        let msg = format!("W8  {:02x} -> {:04x}", val, addr);
        self.log(msg);
    }

    fn store_word(&mut self, addr:u16, val:u16) {
        self.mem_map.store_word(addr,val);
        let msg = format!("W16 {:04x} -> {:04x}", val, addr);
        self.log(msg);
    }

    fn load_word(&self, addr:u16) -> u16 {
        let val = self.mem_map.load_word(addr);

        let msg = format!("R16         {:04x} -> {:04x}", addr, val);
        self.log(msg);
        val
    }
}


