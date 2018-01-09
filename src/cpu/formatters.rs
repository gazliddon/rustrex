use cpu::{Regs, Flags};

use std::fmt;

impl fmt::Display for Regs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:02x} {}", 
            self.pc,
            self.get_d(), 
            self.x,
            self.y,
            self.s,
            self.u,
            self.dp,
            self.flags
            )
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.bits())
    }
}
