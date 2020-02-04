use crate::sig;
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
                    return err(format!("Can't parse signal string: {:?}", sig_string).as_str());
                }
            }
            if let Some(Value::Number(width)) = o.get("width") {
                signal.width = width.as_u64().unwrap_or(1);
            }
            if let Some(Value::String(dir)) = o.get("direction") {
                signal.direction = match dir.as_str() {
                    "in" => Some(In),
                    "out" => Some(Out),
                    "inout" => Some(InOut),
                    _ => {
                        return err(format!("Got a bad signal direction: {:?}", dir).as_str());
                    }
                }
            }
        } else {
            return err("not a signal");
        }
        return err("not a signal");
    }
}
