use crate::types::*;
use serde_json::Value;
use std::slice::Iter;

pub fn tagged_array<'a>(tag: &str, val: &'a Value) -> E<Iter<'a, Value>> {
    if let Value::Array(v) = val {
        if v[0] != Value::String(tag.to_string()) {
            bailfmt!("{} tagged array expects first array element to be the string {}", tag, tag)
        } else {
            Ok(v[1..].iter())
        }
    } else {
        bailfmt!("{:?} tagged array expects array, got:", val.as_str())
    }
}
