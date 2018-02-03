#[derive(PartialEq, PartialOrd)]
enum BreakpointTypes {
    READ8,
    WRITE8,
    READ16,
    WRITE16,
    EXEC,
}

trait BreakpointT {
    fn add_breakpoint(&mut self, addr : u16, kind : BreakpointTypes );
    fn clear_breakpoint(&mut self,  addr : u16, kind : BreakpointTypes );
    fn action(&mut self, addr : u16, kind : BreakpointTypes ) -> bool;
}

#[derive(PartialEq, PartialOrd)]
struct Bp {
    addr : u16,
    kind : BreakpointTypes,
}

struct Breakpoints {
    break_points : Vec<Bp>,
}

impl Breakpoints {
    pub fn new() -> Breakpoints {
        Breakpoints {
            break_points : vec![],
        }
    }

    fn find (&mut self, addr : u16, kind : BreakpointTypes ) -> Option<usize> {
        let this_bp = Bp { addr : addr, kind : kind };
        let mut it = self.break_points.iter();
        it.position(|bp| *bp == this_bp)
    }
}

impl BreakpointT for Breakpoints {
    fn add_breakpoint(&mut self, addr : u16, kind : BreakpointTypes ) {
        let bp = Bp {addr: addr, kind : kind};
        self.break_points.push(bp);
    }

    fn clear_breakpoint(&mut self,  addr : u16, kind : BreakpointTypes ) {
        let exists = self.find(addr, kind);

        if  exists.is_some() {
            self.break_points.remove( exists.unwrap() );
        };
    }

    fn action(&mut self, addr : u16, kind : BreakpointTypes ) -> bool {
        self.find(addr, kind).is_some()
    }
}

