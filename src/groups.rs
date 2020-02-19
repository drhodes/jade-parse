use crate::common::*;
use crate::sig;
use crate::types::*;
use std::collections::HashMap;
use std::str::FromStr;

impl Groups {
    pub fn new() -> Groups {
        Groups { sig_set: HashMap::new() }
    }

    pub fn insert_signals(&mut self, name: String, sigs: Vec<Sig>) -> () {
        self.sig_set.insert(name, sigs);
    }
}
