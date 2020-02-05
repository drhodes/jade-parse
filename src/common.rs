use crate::types::*;
use serde_json::Value;

pub fn tagged_array(tag: &str, val: &Value) -> E<Value> {
    if let Value::Array(v) = val {
        if v.len() != 2 {
            bail!(format!("{} tagged array expects array with two args", tag).as_str())
        } else if v[0] != Value::String(tag.to_string()) {
            bail!(format!("{} tagged array expects first array element to be the string {}", tag, tag).as_str())
        } else {
            Ok(v[1].clone())
        }
    } else {
        bail!(format!("{} tagged array expects array with two args", tag).as_str())
    }
}
