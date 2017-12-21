use cpu::Cpu;
use memmap::MemMap;
use mem::MemoryIO;

struct Machine {
    cpu : Cpu,
    mem : MemMap,
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

    pub fn upload(&mut self, data : &[u8], _address : u16) -> u16 {

        use std::cmp::min;

        let max = 0x10000 - ( _address as usize );

        let to_copy = min(max, data.len());

        if to_copy > 0 {
            let slice = &data[0..to_copy];
            let mut dest = _address;

            for byte in slice {
                self.mem.store_byte(dest, *byte);
                dest = dest + 1;
            }

        }

        to_copy as u16
    }

    pub fn download(&mut self, address : u16, size : u16 ) -> Vec<u8> {
        use std::cmp::min;

        let uaddress = address as usize;

        let mut data: Vec<u8> = Vec::new();
        let max = 0x10000 - uaddress;
        let to_copy = min(max, size as usize);

        if to_copy > 0 {

            for addr in uaddress..(uaddress+to_copy) {
                data.push(self.mem.load_byte(addr as u16));
            }
        }

        data

    }
}
