use cpu2::Cpu;
use mem::MemoryIO;
// use registers::{ Regs};

struct Disassembler<M: MemoryIO> {
    cpu : Cpu<M>,
}

impl <M: MemoryIO> Disassembler<M> {

    pub fn new(cpu : Cpu<M>) -> Self {
        Disassembler {
            cpu : cpu
        }
    }

    pub fn disassemble(&mut self, addr : u16, amount : usize) {
    }

}
