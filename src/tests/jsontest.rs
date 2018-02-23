use mem::{MemoryIO, LoggingMemMap, LogEntry};
use cpu::{Cpu, Regs};
use diss::Disassembler;
use clap::{ArgMatches};

use utils;
use json;
use serde_json;

pub struct JsonTest {
    dont_check_hash : bool,
    json_file : String,
    log_memory: bool
}

impl JsonTest {
    pub fn from_matches( matches : &ArgMatches ) -> JsonTest {
        JsonTest {
            json_file : matches.value_of("JSON FILE").unwrap().to_string(),
            dont_check_hash : matches.is_present("disable-hash-check"),
            log_memory : matches.is_present("log-memory"),
        }
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

pub fn run_json_test(json_test : &JsonTest ) {

    let log_memory = json_test.log_memory;
    let dont_check_hash = json_test.dont_check_hash;
    let json_file = &json_test.json_file;


    println!("Comparing to test run in  {}", json_file);
    println!("Skipping hash check: {}", dont_check_hash);

    let json_contents = utils::load_file_as_string(json_file);

    let run_log : json::RunLog = serde_json::from_str(&json_contents).unwrap();

    // let base_mem = run_log.create_memmap();
    // let mut mem = LoggingMemMap::new(base_mem);

    let mut mem = run_log.create_memmap();

    let mut cpu = Cpu::from_regs(run_log.states[0].regs.clone());

    let mut cycles = 0;

    let mut diss = Disassembler::new();

    let mut it = run_log.states.iter().peekable();

    for i in 0 .. run_log.states.len()/2 {

        // mem.clear_log();

        let log_before = &it.next().unwrap().regs;
        let log_after = &it.peek().unwrap();

        let log_regs_after = &log_after.regs;

        let prev_sim = cpu.regs.clone();

        let pc = cpu.regs.pc;

        let ins = cpu.step(&mut mem);

        let sim = &cpu.regs;

        let is_hash_ok = if dont_check_hash {
            true
        } else {
            match log_after.digest {
                Some(ref d) => {
                    let hash = mem.get_sha1_string();
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

            let (ins, txt) =  diss.diss(&mem, pc, None);
            // let writes_str = get_writes_as_str(&mem);
            // println!("{:04x}   {:20}{:20} : {}", pc, txt, writes_str, sim);

            let (ins, txt) =  diss.diss(&mem, cpu.regs.pc, None);
            println!("");

            println!("Next op:");
            println!("{:04x}   {:20}", cpu.regs.pc, txt);

            println!("");

            if is_hash_ok == false {
                let hash = mem.get_sha1_string();
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
    println!("Successfully run {} instructions", run_log.states.len());
}
