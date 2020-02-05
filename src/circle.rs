use crate::common::*;
use crate::types::*;
use serde_json::Value;
use std::slice::Iter;

impl Circle {
    pub fn from_value(val: Value) -> E<Circle> {
        let mut val = bailif!(tagged_array("circle", &val), "Circle::from_value failes to decode")?;
        if let Some(Value::Array(v)) = val.next() {
            if v.len() != 4 {
                bail!(format!("circle expected array of 4 numbers, got: {:?}", v).as_str())
            } else {
                let x: u32 = serde_json::from_value(v[0].clone())?;
                let y: u32 = serde_json::from_value(v[1].clone())?;
                let rot: Rot = serde_json::from_value(v[2].clone())?;
                let radius: f64 = serde_json::from_value(v[3].clone())?;
                Ok(Circle { x, y, rot, radius })
            }
        } else {
            bail!(format!("circle expected array, got: {:?}", val).as_str())
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
    fn circle1() {
        let expect = Circle { x: 0, y: 0, rot: Rot0, radius: 3.6 };
        let val = json!(["circle", [0, 0, 0, 3.6]]);
        let got: Circle = Circle::from_value(val).unwrap();
        assert_eq!(expect, got);
    }

    #[test]
    fn circle2() {
        let val = json!(["circle", [3.5, 0, 0, 3.6]]);
        let got = Circle::from_value(val);
        assert!(got.is_err());
    }
}
