use serde_json::Value;

use crate::common::*;
use crate::types::*;

impl Jumper {
    pub fn from_value(val: &Value) -> E<Jumper> {
        let mut val_iter = bailif!(tagged_array("jumper", &val), "Jumper::from_value failes to decode")?;

        let coord3: Coord3 = match val_iter.next() {
            Some(c) => serde_json::from_value::<Coord3>(c.clone())?,
            None => {
                return bailfmt!("Jumper expects 1 element, a 3 number array, got: {:?}", val);
            }
        };
        Ok(Jumper { coord3 })
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
    fn jumper1() {
        let val = json!(["jumper", [0, 0, 0]]);
        let got = Jumper::from_value(&val);
        assert!(got.is_ok());
    }
}
