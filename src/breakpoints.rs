#[derive(Clone, Debug,PartialEq, PartialOrd)]
pub enum BreakPointTypes {
    READ,
    WRITE,
    EXEC,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BreakPoint {
    addr : u16,
    kind : BreakPointTypes,
}

impl BreakPoint {
    pub fn new( kind : BreakPointTypes, addr : u16 ) -> BreakPoint {
        BreakPoint {
            kind,
            addr,
        }
    }

}

struct BreakPoints {
    break_points : Vec<BreakPoint>,
}

impl BreakPoints {

    pub fn new() -> BreakPoints {
        BreakPoints {
            break_points : vec![],
        }
    }

    fn find(&mut self, b : &BreakPoint) -> Option<usize> {
        let mut it = self.break_points.iter();
        it.position(|bp| *bp == *b)
    }

    fn add_breakpoint(&mut self, b : &BreakPoint ) {
        if let Some(_) = self.find(b) {
            self.clear_breakpoint(b)
        }

        self.break_points.push(b.clone());
    }

    fn clear_breakpoint(&mut self,  b : &BreakPoint) {
        if let Some(i) = self.find(b) {
            self.break_points.remove(i);
        }
    }

    fn clear_breakpoints_at_addr(&mut self, _addr : u16) {
        panic!("tbd")
    }

}


