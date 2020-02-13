use serde_json::Value;

use crate::common::*;
use crate::types::*;

impl Schematic {
    pub fn from_value(val: &Value) -> E<Schematic> {
        let arr: &Vec<Value> = if let Value::Array(arr) = val {
            arr
        } else {
            return bailfmt!("Schematic::from_value got bad json value: {:?}", val);
        };

        let mut parts = vec![];
        for part in arr {
            let p = bailif!(Part::from_value(part), "Schematic got a bad part")?;
            parts.push(p);
        }

        Ok(Schematic { parts })
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
    fn schematic1() {
        let val = json!(
            [["/gates/xor2",[152,128,1]],
             ["/gates/xor2",[80,128,1]],
             ["/gates/xor2",[8,128,1]],
             ["/gates/xor2",[-64,128,1]],
             ["wire", [-144,-88,0,0,24]],
             ["wire", [136,88,0,0,40]],
             ["wire", [80,128,0,0,-48]],
             ["wire", [-96,-40,0,16,0]],
             ["wire", [-80,128,0,0,-168]],
             ["wire", [-80,-192,0,0,152],{"signal":"A[3]"}],
             ["wire", [-96,-56,0,88,0]],
             ["wire", [-8,24,0,0,-80]],
             ["wire", [-8,-192,0,0,136],{"signal":"A[2]"}],
             ["wire", [-96,-72,0,160,0]],
             ["wire", [64,8,0,0,-80]],
             ["wire", [-96,-88,0,232,0]],
             ["wire", [136,-192,0,0,104],{"signal":"A [0]"}],
             ["wire", [136,-8,0,0,-80]],
             ["wire", [152,-24,0,0,-80]],
             ["wire", [152,-104,0,32,0],{"signal":"Ci"}],
             ["wire", [-192,-96,0,-8,0],{"signal":"Co"}],
             ["wire", [64,-72,0,0,-120],{"signal":"A[1]"}],
             ["wire", [144,176,0,0,8],{"signal":"vout [0]"}],
             ["wire", [72,176,0,0,8],{"signal":"vout[1]"}],
             ["wire", [0,176,0,0,8],{"signal":"vout [2]"}],
             ["wire", [-72,176,0,0,8],{"signal":"vout[3]"}],
             ["wire", [-144,-104,0,296,0]],
             ["text", [-158,-230,0],{"text":"w/fast, delta_t = 1.777ns - 1.361ns  = -.416ns","font":"12pt sans-serif"}],
             ["/gates/and2", [128,72,4]],
             ["/gates/and3", [56,32,4]],
             ["/gates/and4", [-16,-24,4]],
             ["/gates/and4", [-96,-88,4]],
             ["/gates/and2", [-144,-104,4]]]
        );
        match Schematic::from_value(&val) {
            Err(e) => panic!("{:?}", e),
            Ok(scm) => {
                // are all the parts there?
                const NUM_PARTS: usize = 33;
                assert_eq!(scm.parts.len(), NUM_PARTS);
            }
        }
    }

    #[test]
    fn schematic2() {
        let val = json!(["this should fail because module name doesn't start with a slash", [0, 0, 0]]);
        let got = Schematic::from_value(&val);

        if got.is_ok() {
            panic!("{:?}", got);
        }
    }
}
