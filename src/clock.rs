pub  struct Clock {
    cycles : u64,
    cycles_per_second : u64,
}

impl Clock {
    fn new(cycles_per_second : u64) -> Clock {
        Clock {
            cycles: 0,
            cycles_per_second : cycles_per_second
        }
    }

    fn add_cycles(&mut self, v : u32) -> u64 {
        let r = self.cycles.wrapping_add(v as u64);
        self.cycles = r;
        r
    }

    fn inc(&mut self) -> u64 {
        self.add_cycles(1)
    }

    fn cycles_to_seconds(&self, cycles : u64) -> f64  {
        // TODO
        0.0
    }

    fn seconds_to_cycles(&self, seconds : u64) -> u64  {
        // TODO
        0
    }
}

