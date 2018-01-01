use cpu::Cpu;
use memmap::MemMap;

struct Disassmbler {
    pub cpu : Cpu,
    pub mem : MemMap,
}

impl Disassmbler {

    pub fn new(cpu : Cpu, mem : MemMap) -> Self {
        Disassmbler {
            cpu : cpu,
            mem :mem
        }
    }

    pub fn disassemble(&self) -> (String, u16) {

        (String::from("ksjakjsak"), 10)

    }

}
