use mem::{MemoryIO, LoggingMemMap, LogEntry, MemMap};
use cpu::{Cpu, Regs};
use diss::Disassembler;
use clap::{ArgMatches};

use tests::tester;
use proclog::{Step};

use utils;
use serde_json;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MemInit {
    pub base : u16,
    pub size : usize,
    pub writeable : bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct RunLog {
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

pub struct JsonTest {

    dont_check_hash : bool,
    json_file : String,
    log_memory: bool,

    mem : MemMap,
    cpu : Cpu,
    // run_log : json::RunLog,
    steps : Vec<Step>,

}

impl tester::Tester for JsonTest {

    fn from_matches( matches : &ArgMatches ) -> JsonTest {

        let json_file = matches.value_of("JSON FILE").unwrap().to_string();

        println!("Loading: {}", json_file);

        let json_contents = utils::load_file_as_string(&json_file);

        println!("Converting from json");

        let run_log : RunLog = serde_json::from_str(&json_contents).unwrap();

        println!("Done, {} steos to emulate", run_log.states.len());

        let r = JsonTest {
            json_file : json_file,
            dont_check_hash : matches.is_present("disable-hash-check"),
            log_memory : matches.is_present("log-memory"),
            mem : run_log.create_memmap(),
            cpu : Cpu::from_regs(run_log.states[0].regs.clone()),
            steps : run_log.states,
        };

        r
    }

    fn run(&mut self) {

        println!("Comparing to test run in {}", self.json_file);
        println!("Skipping hash check: {}", self.dont_check_hash);

        // let base_mem = run_log.create_memmap();
        // let mut mem = LoggingMemMap::new(base_mem);

        let mut cycles = 0;

        let mut diss = Disassembler::new();

        let mut it = self.steps.iter().peekable();

        for i in 0 .. self.steps.len()/2 {

            // mem.clear_log();

            let log_before = &it.next().unwrap().regs;
            let log_after = &it.peek().unwrap();

            let log_regs_after = &log_after.regs;

            let prev_sim = self.cpu.regs.clone();

            let pc = self.cpu.regs.pc;

            let ins = self.cpu.step(&mut self.mem);

            let sim = &self.cpu.regs;

            let is_hash_ok = if self.dont_check_hash {
                true
            } else {
                match log_after.digest {
                    Some(ref d) => {
                        let hash = self.mem.get_sha1_string();
                        hash == *d
                    }
                    _ => true,
                }
            };

            // let (ins, txt) =  diss.diss(&mem, pc, None);
            // let writes_str = get_writes_as_str(&mem);
            // println!("{:04x}   {:20}{:20} : {}", pc, txt, writes_str, sim);

            if ( sim != log_regs_after ) | !is_hash_ok {

                println!("Error after {} instructions", i);

                let (ins, txt) =  diss.diss(&self.mem, pc, None);
                // let writes_str = get_writes_as_str(&mem);
                // println!("{:04x}   {:20}{:20} : {}", pc, txt, writes_str, sim);

                let (ins, txt) =  diss.diss(&self.mem, self.cpu.regs.pc, None);
                println!("");

                println!("Next op:");
                println!("{:04x}   {:20}", self.cpu.regs.pc, txt);

                println!("");

                if is_hash_ok == false {
                    let hash = self.mem.get_sha1_string();
                    // let log_hash = state_after.digest.unwrap().clone();
                    println!("       sim: {}", hash);
                    // println!(" should be: {}", log_hash);
                }

                println!("");

                println!("            {}", Regs::get_hdr());
                println!("      prev: {}", prev_sim);
                println!("       sim: {}", sim);
                println!(" should be: {}", log_regs_after);

                println!("");

            } 

            cycles = cycles + 1;
        }
        println!("Successfully run {} instructions", self.steps.len());
    }
}


fn get_writes_as_str( mem : &LoggingMemMap ) -> String {

    let writes : Vec<LogEntry>= mem.get_log()
        .into_iter()
        .filter(|msg| msg.write)
        .collect();

    if writes.len() != 0 {
        writes[0].to_string()
    } else {
        "".to_string()
    }
}

