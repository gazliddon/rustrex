use mem::MemoryIO;

#[derive(Default)]
#[derive(Debug)]
pub struct InstructionDecoder {
    pub op_code : u16,
    pub cycles : usize,
    pub addr : u16,
    pub bytes : usize,
    pub next_addr : u16,
    pub mem : [u8; 4],
    pub operand : u16,
}



impl InstructionDecoder {

    pub fn new(addr: u16)-> Self {
        InstructionDecoder {
            addr : addr,
            next_addr : addr,
            .. Default::default()
        }
    }

    pub fn inc_cycles(&mut self) -> usize {
        self.cycles = self.cycles + 1;
        self.cycles
    }

    pub fn fetch_byte<M : MemoryIO>(&mut self, mem: &M) -> u8 {
        let b = mem.load_byte(self.next_addr);
        self.next_addr = self.next_addr.wrapping_add(1);

        self.mem[self.bytes] = b;
        self.bytes = self.bytes + 1;

        b
    }

    pub fn fetch_byte_as_i8<M : MemoryIO>(&mut self, mem: &M) -> i8 {
        self.fetch_byte(mem) as i8
    }

    pub fn fetch_byte_as_i16<M : MemoryIO>(&mut self, mem: &M) -> i16 {
        self.fetch_byte_as_i8(mem) as i16 
    }

    pub fn fetch_word_as_i16<M : MemoryIO>(&mut self, mem: &M) -> i16 {
        self.fetch_word(mem) as i16
    }

    pub fn fetch_word<M : MemoryIO>(&mut self, mem: &M) -> u16 {
        let w = mem.load_word(self.next_addr);
        self.next_addr = self.next_addr.wrapping_add(2);

        self.mem[self.bytes] = ((w >> 8) & 0xff) as u8;
        self.mem[self.bytes+1] = w as u8;
        self.bytes = self.bytes + 2;
        w   
    }

    pub fn get_next_addr(&self) -> u16 {
        self.next_addr
    }

    pub fn fetch_instruction<M: MemoryIO>(&mut self, mem: &M) -> u16 {

        let a = self.fetch_byte(mem) as u16;

        self.op_code = match a {
            0x10 | 0x11 => {
                (a << 8) + self.fetch_byte(mem) as u16
            }
            _ => a
        };

        self.op_code
    }
}
