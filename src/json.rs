
use cpu::Regs;
use mem::{MemMap, MemoryIO };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MemInit {
    base : u16,
    size : usize,
    writeable : bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
    cycles : usize,
    digest : String,
    regs: Regs,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunLog {
    file_name : String,
    load_addr : u16,
    memory : Vec<MemInit>,
    states : Vec<State>,
}

impl RunLog {

    pub fn create_memmap(&self) -> MemMap {

        let full_file_name = format!("cpp/{}", self.file_name);

        let mut m = MemMap::new();

        use utils::{load_file};

        for mb in &self.memory {
            m.add_mem_block("NO NAME", mb.writeable, mb.base, mb.size)
        };

        let data = load_file(&full_file_name);
        let addr = self.load_addr;
        m.upload(addr, &data);

        m
    }
}
