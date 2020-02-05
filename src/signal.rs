use crate::sig;

#[macro_use]
use crate::types::*;
use serde_json::Value;

impl Signal {
    pub fn from_value(val: Value) -> E<Signal> {
        if let Value::Object(o) = val {
            let mut signal = Signal::default();

            if let Some(Value::String(sig_string)) = o.get("signal") {
                if let Some(sig) = sig::parse_sig(sig_string) {
                    signal.sig = Some(sig);
                } else {
                    return bailfmt!("Can't parse signal string: {:?}", sig_string);
                }
            }
            if let Some(Value::Number(width)) = o.get("width") {
                signal.width = width.as_u64()
            }
            if let Some(dir) = o.get("direction") {
                signal.direction = Some(Direction::from_value(dir.clone())?);
            }
            return Ok(signal);
        } else {
            return bailfmt!("in signal parse, expected object, got: {:?}", val);
        }
    }
}

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;
    use serde_json::json;

    #[test]
    fn signal1() {
        let val = json!({"signal":"out[2:0]","direction":"out"});
        let got = Signal::from_value(val).unwrap();
        let expected =
            Signal { sig: Some(Sig::SigRange("out".to_string(), 2, 0)), width: None, direction: Some(Out) };
        assert_eq!(got, expected);
    }

    #[test]
    fn signal2() {
        let val = json!({"signal":"out[2:0]"});
        let got = Signal::from_value(val).unwrap();
        let expected =
            Signal { sig: Some(Sig::SigRange("out".to_string(), 2, 0)), width: None, direction: None };
        assert_eq!(got, expected);
    }

    #[test]
    fn signal3() {
        let val = json!({"signal":"out[2:0]"});
        let got = Signal::from_value(val).unwrap();
        let expected =
            Signal { sig: Some(Sig::SigRange("out".to_string(), 2, 0)), width: None, direction: None };
        assert_eq!(got, expected);
    }
}
