use serde_json::Value;

use crate::common::*;
use crate::types::*;

impl Port {
    pub fn from_value(val: &Value) -> E<Port> {
        let mut val_iter = bailif!(tagged_array("port", &val), "Port::from_value failes to decode")?;

        let coord3: Coord3 = match val_iter.next() {
            Some(c) => serde_json::from_value::<Coord3>(c.clone())?,
            None => {
                return bailfmt!("Port expects 2 elements, a location and properties object, got: {:?}", val);
            }
        };

        let signal = match val_iter.next() {
            Some(val) => Some(Signal::from_value(&val)?),
            None => None,
        };
        Ok(Port { coord3, signal })
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
    fn port1() {
        let val = json!(["port", [0,0,0], {"signal": "A[3:0]"}]);
        let got = Port::from_value(&val);
        assert!(got.is_ok());
    }

    #[test]
    fn port2() {
        let val = json!(["port", [0, 0, 0]]);
        let got = Port::from_value(&val);
        assert!(got.is_ok());
    }
}
