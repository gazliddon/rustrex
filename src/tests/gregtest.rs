use clap::{ArgMatches};

impl GregTest {
    pub fn from_matches( matches : &ArgMatches ) -> GregTest {
        GregTest {
            log_file : matches.value_of("LOG FILE").unwrap().to_string(),
        }
    }
}

pub struct GregTest {
    pub log_file : String,
}

pub fn run_greg_test(greg_test : &GregTest) {
}

