use crate::types::*;
use serde_json::Value;

// ["wire" Coord5 Signal?]

impl Wire {
    pub fn from_value(val: &Value) -> E<Wire> {
        if let Value::Array(xs) = val.clone() {
            if xs[0] != "wire" {
                return bail!("not a wire");
            }
            let coord5 = serde_json::from_value(xs[1].clone())?;
            if xs.len() == 2 {
                return Ok(Wire { coord5, signal: None });
            }
            if xs.len() == 3 {
                let s = Signal::from_value(&val[2])?;
                return Ok(Wire { coord5, signal: Some(s) });
            }
            return bailfmt!("HUH. json wire array has more than 3 elements: {:?}", xs.len());
        } else {
            return bail!("not a wire");
        }
    }
}

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn wire1() {
        let val = json!( ["wire", [-80,-192,0,0,152],{"signal":"A[3]"}] );
        let got = Wire::from_value(&val);
        if got.is_err() {
            panic!("{:?}", got)
        }
    }

    #[test]
    fn wire2() {
        let val = json!(["wire", [136,-192,0,0,104],{"signal":"A [0]"}]);
        let got = Wire::from_value(&val);
        if got.is_err() {
            panic!("{:?}", got)
        }
    }
}
