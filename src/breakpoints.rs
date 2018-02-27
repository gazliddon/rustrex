#[derive(Clone, Debug,PartialEq, PartialOrd)]
enum BreakpointTypes {
    READ,
    WRITE,
    EXEC,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BreakPoint {
    addr : u16,
    kind : BreakpointTypes,
}

struct Breakpoints {
    break_points : Vec<BreakPoint>,
}

impl Breakpoints {

    pub fn new() -> Breakpoints {
        Breakpoints {
            break_points : vec![],
        }
    }

    fn find(&mut self, b : &BreakPoint) -> Option<usize> {
        let mut it = self.break_points.iter();
        it.position(|bp| *bp == *b)
    }

    fn add_breakpoint(&mut self, b : &BreakPoint ) {
        if let Some(x) = self.find(b) {
            self.clear_breakpoint(b)
        }

        self.break_points.push(b.clone());
    }

    fn clear_breakpoint(&mut self,  b : &BreakPoint) {
        if let Some(i) = self.find(b) {
            self.break_points.remove(i);
        }
    }

    fn clear_breakpoints_at_addr(&mut self, addr : u16) {
        panic!("tbd")
    }

}


