use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

use serde_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SymbolTable {
    syms_to_val : BTreeMap<String, u16>,
}

fn load_file_as_string(file_name : &String ) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

impl SymbolTable {
    pub fn new(file_name : &'static str) -> Self {
        let s = load_file_as_string(&file_name.to_string());
        let v : BTreeMap<String,u16> = serde_yaml::from_str(&s).unwrap(); 

        SymbolTable {
            syms_to_val : v
        }
    }

    pub fn add(&mut self, name : String, val : u16 ) -> &mut Self {
        self.syms_to_val.insert(name, val);
        self
    }

    pub fn lookup_from_val(&self, addr : u16) -> Option<String> {
        for (key, value) in self.syms_to_val.iter() {
            if *value == addr {
                return Some(key.clone());
            }
        }
        None
    }

    pub fn lookup(&self, name : &String) -> Option<u16> {
        match self.syms_to_val.get(name) {
            Some(u16ref) => Some(*u16ref),
            None => None,
        }
    }
}

