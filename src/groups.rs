use crate::types::*;
use std::collections::HashMap;

impl Groups {
    pub fn new() -> Groups {
        Groups { sig_set: HashMap::new() }
    }

    pub fn insert_signals(&mut self, name: String, sigs: Vec<Sig>) -> () {
        self.sig_set.insert(name, sigs);
    }
}
