#![feature(plugin)]
#![plugin(clippy)]

#![allow(suspicious_arithmetic_impl)]
#![allow(redundant_field_names)]
#![allow(cast_lossless)]

#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;

extern crate serde_yaml;
extern crate serde_json;
extern crate sha1;
extern crate separator;


extern crate regex;
extern crate num;
extern crate clap;

#[macro_use] extern crate log;

#[macro_use] mod cpu;

mod mem;
mod symtab;
mod utils;
mod diss;
mod proclog;
mod breakpoints;
mod tests; 
mod timer;
mod gdbstub;

mod m6522;
mod clock;
mod vectrex;

use tests::{GregTest, JsonTest, Tester};
use clap::{Arg, App, SubCommand, ArgMatches};

fn do_test<T : Tester>(matches : &ArgMatches) -> T{
    let mut tester = T::from_matches(matches);
    tester.run();
    tester
}

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
                         .index(1)
                         .help("json log file to load"))
                    .arg(Arg::with_name("show-disassembly")
                         .short("s")
                         .long("show-disassembly")
                         .help("show disassembly"))
                    .arg(Arg::with_name("check-cycles")
                         .short("c")
                         .long("check-cycles")
                         .help("make sure cycle timings are accurate"))
                    .arg(Arg::with_name("no-hash-check")
                         .short("n")
                         .long("no-hash-check")
                         .help("disable memory hash testing"))
                    .arg(Arg::with_name("log-memory")
                         .short("l")
                         .long("log-memory")
                         .help("enable memory logging")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("greg") {
        do_test::<GregTest>(matches);
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        do_test::<JsonTest>(matches);
    }
}

////////////////////////////////////////////////////////////////////////////////


