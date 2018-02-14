#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate sha1;

extern crate regex;
#[macro_use] mod cpu;

mod mem;
mod via;
mod symtab;
mod utils;
mod diss;
mod proclog;
mod breakpoints;
mod json;

// use proclog::{Step2};
// use symtab::SymbolTable;
use mem::{MemoryIO, LoggingMemMap, LogEntry};
use cpu::{Cpu};
use diss::Disassembler;

fn main() {

    let json_file = "cpp/out.json";
    let json_contents = utils::load_file_as_string(json_file);
    let run_log : json::RunLog = serde_json::from_str(&json_contents).unwrap();

    let base_mem = run_log.create_memmap();

    let mut mem = LoggingMemMap::new(base_mem);

    let mut cpu = Cpu::from_regs(run_log.states[0].regs.clone());

    let mut cycles = 0;

    let mut diss = Disassembler::new();

    let mut it = run_log.states.iter().peekable();

    for i in 0 .. run_log.states.len()/2 {

        let log_before = &it.next().unwrap().regs;

        let state_after = &it.peek().unwrap();

        let log_regs_after = &state_after.regs;

        let log_hash_after = &state_after.digest;

        let (ins, txt) =  diss.diss(&mem, cpu.regs.pc, None);

        let prev_sim = cpu.regs.clone();

        mem.clear_log();

        let ins = cpu.step(&mut mem);

        let sim = &cpu.regs;

        let writes : Vec<LogEntry>= mem.get_log()
            .into_iter()
            .filter(|msg| msg.write)
            .collect();

        let writes_str = if writes.len() != 0 {
            writes[0].to_string()
        } else {
            "".to_string()
        };;

        let hash = mem.get_sha1_string();
        let hash_ok = hash == *log_hash_after;

        println!("{:04x}   {:20}{:20} : {:5} : {}", cpu.regs.pc, txt, writes_str, hash_ok,  sim);

        if sim != log_regs_after {

            println!("");

            println!("            {}", cpu::Regs::get_hdr());
            println!("      prev: {}", prev_sim);
            println!("       sim: {}", sim);
            println!(" should be: {}", log_regs_after);

            println!("");

            for msg in mem.get_log() {
                println!("{}", msg);
            }

            println!("");
            panic!("");
        } 

        cycles = cycles + 1;
    }

    // for step in steps {

    //     let (ins, txt) =  diss.diss(&mem, cpu.regs.pc, None);

    //     let hash = mem.get_sha1_string();

    //     // println!("digest: {}",hash);

    //     mem.clear_log();

    //     let prev_sim = cpu.regs.clone();

    //     let ins = cpu.step(&mut mem);

    //     let sim = &cpu.regs;

    //     let log = &step.regs_after;

    // }

}

