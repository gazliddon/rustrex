use cpu::Cpu;
use isa::Ins;
use isa::get_ins;
use memmap::MemMap;
use mem::MemoryIO;
use mem::to_mem_range;
// use addr::format_operand;
// use addr::fetch_operand;


#[derive(Debug)]
pub struct Machine {
    pub cpu : Cpu,
    pub mem : MemMap,
}




impl Machine {

    pub fn new() -> Machine {

        let mem = MemMap::new();
        let cpu = Cpu::new();

        Machine {
            cpu : cpu,
            mem : mem,
        }
    }



    pub fn fetch_instruction(&self, addr : u16 )  -> &'static Ins {
        let mut op_code = self.mem.load_byte(addr) as u16;

        op_code = match op_code {
            0x10 => self.mem.load_word(addr),
            0x11 => self.mem.load_word(addr),
            _ => op_code,
        };

        let instruction = get_ins(op_code);

        instruction
    }

    pub fn disassemble(&mut self, addr : u16 ) -> (u16, String) {
        // let ins = self.fetch_instruction(addr);

        // let (_, operand) = fetch_operand(&ins.addr_mode.mode,
        //                                  ins.op_code,
        //                                  &mut self.cpu,
        //                                  &mut self.mem,
        //                                  addr + ins.operand_offset as u16);

        // let bytes = ins.bytes;
        // let next_addr = (addr as u32 + bytes as u32) as u16;
        // let formatted = format_operand(&ins.addr_mode.mode, operand);
        // (next_addr, format!("{} {} ({})", ins.op.mnenomic, formatted, ins.addr_mode.name))
        
        (0, String::from("kjsakjsa"))
    }

    pub fn upload(&mut self, data : &[u8], _address : u16) {
        let range = to_mem_range(_address, data.len() as u16);

        for addr in range {
            self.mem.store_byte(addr, data[( addr - _address ) as usize]);
        }
    }

    pub fn download(&mut self, _address : u16, size : u16 ) -> Vec<u8> {

        let range = to_mem_range(_address, size);

        let mut data : Vec<u8> = Vec::new();

        for addr in range {
            let b = self.mem.load_byte(addr);
            data.push(b);
        }

        data
    }
}
