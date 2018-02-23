#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;

extern crate serde_yaml;
extern crate serde_json;
extern crate sha1;

extern crate regex;
extern crate ilog2;
extern crate num;
extern crate cpuprofiler;
extern crate clap;

use clap::{Arg, App, SubCommand};

#[macro_use] 
mod cpu;

mod mem;
mod via;
mod symtab;
mod utils;
mod diss;
mod proclog;
mod breakpoints;
mod json;
mod tests; 

use tests::{GregTest, run_greg_test, JsonTest, run_json_test};

fn main() {

    let matches = App::new("Vectrex Emulator")

        .version("0.1")
        .author("Gazaxian")
        .about("Rust Vectrex emulator")

        .subcommand(SubCommand::with_name("emu")
                    .arg(Arg::with_name("enable-gdb")
                         .short("g")
                         .long("enable-gdb")
                         .help("Enable GDB debugging"))
                    .arg(Arg::with_name("ROM FILE")
                         .required(true)
                         .index(1)
                         .help("Set the ROM file")))

        .subcommand(SubCommand::with_name("greg")
                    .arg(Arg::with_name("LOG FILE")
                         .required(true)
                         .index(1)
                         .help("Set the ROM file"))
                    .arg(Arg::with_name("log-memory")
                         .short("l")
                         .long("log-memory")
                         .help("enable memory logging"))
                    .arg(Arg::with_name("num-instructions")
                         .short("n")
                         .long("num-instructions")
                         .help("number of instructions (default 100)")))

        .subcommand(SubCommand::with_name("test")
                    .arg(Arg::with_name("JSON FILE")
                         .required(true)
                         .value_name("JSON")
                         .help("JSON log file to load"))
                    .arg(Arg::with_name("disable-hash-check")
                         .short("d")
                         .long("disable-hash-check")
                         .help("disable memory hash testing"))
                    .arg(Arg::with_name("log-memory")
                         .short("l")
                         .long("log-memory")
                         .help("enable memory logging")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("greg") {
        let greg_test = GregTest::from_matches(&matches);
        run_greg_test(&greg_test);
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        let json_test = JsonTest::from_matches(&matches);
        run_json_test(&json_test);
    }
}

////////////////////////////////////////////////////////////////////////////////


