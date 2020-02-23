use serde_json::Value;

use crate::types::*;

impl Icon {
    pub fn from_value(val: &Value) -> E<Icon> {
        let arr: &Vec<Value> = if let Value::Array(arr) = val {
            arr
        } else {
            return bailfmt!("Icon::from_value got bad json value: {:?}", val);
        };

        let mut parts = vec![];
        for part in arr {
            let p = bailif!(IconPart::from_value(part), "Icon got a bad part")?;
            parts.push(p);
        }

        Ok(Icon { parts })
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
    fn icon1() {
        let val = json!(
            [["line", [-24,-24,0,48,0]],
             ["line", [24,-24,0,0,48]],
             ["line", [24,24,0,-48,0]],
             ["line", [-24,24,0,0,-48]],
             ["text", [-13,-9,0],{"text":"GarrInc"}],
             ["terminal", [32,0,4],{"name":"Ci"}],
             ["terminal", [-32,0,0],{"name":"Co"}],
             ["terminal", [0,-32,1],{"name":"A[3:0]"}],
             ["terminal", [0,32,3],{"name":"vout [3:0]"}],
             ["text", [-8,-21,0],{"text":"A[3:0]","font":"4pt sans-serif"}],
             ["text", [-22,0,0],{"text":"Co","font":"4pt sans-serif"}],
             ["text", [17,0,0],{"text":"Ci","font":"4pt sans-serif"}],
             ["text", [-10,20,0],{"text":"out[3:0]","font":"4pt sans-serif"}],
             ["text", [-8,6,0],{"text":"4","font":"18pt sans-serif"}]]        );

        match Icon::from_value(&val) {
            Err(e) => panic!("{:?}", e),
            Ok(scm) => {
                // are all the parts there?
                const NUM_PARTS: usize = 14;
                assert_eq!(scm.parts.len(), NUM_PARTS);
            }
        }
    }
}
