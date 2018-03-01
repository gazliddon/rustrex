pub  struct StandardClock {
    cycles : u64,
    cycles_per_second : u64,
}

trait Clock {
    fn cycles_per_second(&self) -> u64;
    fn add_cycles(&mut self, v : usize) -> u64;

    fn inc(&mut self) -> u64 {
        self.add_cycles(1)
    }
}

impl StandardClock {
    pub fn new(cycles_per_second : u64) -> Self {
        Self {
            cycles: 0,
            cycles_per_second
        }
    }
}

impl Clock for StandardClock {
    fn cycles_per_second(&self) -> u64 {
        self.cycles_per_second
    }

    fn add_cycles(&mut self, v : usize) -> u64 {
        let r = self.cycles.wrapping_add(v as u64);
        self.cycles = r;
        r
    }
}

