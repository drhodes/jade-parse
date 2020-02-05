use crate::types::*;
use serde_json::Value;

// ["wire" Coord5 Signal?]

impl Wire {
    pub fn from_value(val: Value) -> E<Wire> {
        if let Value::Array(xs) = val.clone() {
            if xs[0] != "wire" {
                return bail!("not a wire");
            }
            let coord5 = serde_json::from_value(xs[1].clone())?;
            if xs.len() == 2 {
                return Ok(Wire { coord5, signal: None });
            }
            if xs.len() == 3 {
                let s = Signal::from_value(val)?;
                return Ok(Wire { coord5, signal: Some(s) });
            }
            return bailfmt!("HUH. json wire array has more than 3 elements: {:?}", xs.len());
        } else {
            let e: E<Wire> = bail!("not a wire");
            return bail!("not a wire");
        }
    }
}
