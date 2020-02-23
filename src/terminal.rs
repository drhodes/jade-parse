use serde_json::Value;

use crate::common::*;
use crate::types::*;

impl Terminal {
    pub fn from_value(val: &Value) -> E<Terminal> {
        let mut val_iter = bailif!(tagged_array("terminal", &val), "Terminal::from_value failes to decode")?;

        let coord3: Coord3 = match val_iter.next() {
            Some(c) => serde_json::from_value::<Coord3>(c.clone())?,
            None => {
                return bailfmt!("Terminal expects 2 elements, a location and properties object, got: {:?}", val);
            }
        };

        let sig = match val_iter.next() {
            Some(val @ Value::String(_)) => Sig::from_value(val.clone())?,
            Some(Value::Object(obj)) => match obj.get("name") {
                Some(sig_val) => Sig::from_value(sig_val.clone())?,
                _ => {
                    return bailfmt!("could't find signal name in this terminal: {:?}", val);
                }
            },
            x => bailfmt!("found a corrupted terminal item in schematic: {:?}", x)?,
        };
        Ok(Terminal { coord3, sig })
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
    fn terminal1() {
        let val = json!(["terminal", [0, 0, 0], "A[3:0]"]);
        let got = Terminal::from_value(&val);
        assert!(got.is_ok());
    }

    #[test]
    fn terminal2() {
        let val = json!(["terminal", [0, 0, 0, 0], "A[3:0]"]);
        let got = Terminal::from_value(&val);
        assert!(got.is_err());
    }

    #[test]
    fn terminal3() {
        let val = json!(["terminal", [0, 0, 0], ""]);
        let got = Terminal::from_value(&val);
        assert!(got.is_err());
    }

    #[test]
    fn terminal4() {
        let val = json!(["terminal", [32,0,4],{"name":"Ci"}]);
        let got = Terminal::from_value(&val);
        assert!(got.is_ok());
    }
}
