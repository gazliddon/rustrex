
use mem::{MemMap, MemoryIO };
use proclog::{Step};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MemInit {
    pub base : u16,
    pub size : usize,
    pub writeable : bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunLog {
    pub file_name : String,
    pub load_addr : u16,
    pub memory : Vec<MemInit>,
    pub states : Vec<Step>,
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
