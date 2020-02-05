use crate::types::*;
use serde_json::Value;
use std::slice::Iter;

pub fn tagged_array<'a>(tag: &str, val: &'a Value) -> E<Iter<'a, Value>> {
    if let Value::Array(v) = val {
        if v.len() != 2 {
            bail!(format!("{} tagged array expects array with two args", tag).as_str())
        } else if v[0] != Value::String(tag.to_string()) {
            bail!(format!("{} tagged array expects first array element to be the string {}", tag, tag).as_str())
        } else {
            Ok(v[1..].iter())
        }
    } else {
        bail!(format!("{} tagged array expects array with two args", tag).as_str())
    }
}
