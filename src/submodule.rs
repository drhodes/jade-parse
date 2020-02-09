use serde_json::Value;

use crate::common::*;
use crate::types::*;

impl SubModule {
    pub fn from_value(val: &Value) -> E<SubModule> {
        let arr: &Vec<Value> = if let Value::Array(arr) = val {
            arr
        } else {
            return bailfmt!("SubModule::from_value got bad json value: {:?}", val);
        };

        if arr.len() != 2 {
            return bailfmt!("submodule expected an array of size 2, got: {:?}", arr);
        }

        let name: String = match &arr[0] {
            Value::String(s) => {
                if !s.starts_with("/") {
                    return bailfmt!("submodule has fishy name, should start with '/', got: {}", s);
                }
                s.clone()
            }
            _ => return bailfmt!("submodule expected module name as first arg, got: {:?}", arr[0]),
        };

        if let Ok(coord3) = serde_json::from_value::<Coord3>(arr[1].clone()) {
            return Ok(SubModule { name, coord3 });
        } else {
            return bailfmt!("submodule expects 2 elements, a location and properties object, got: {:?}",
                            arr[1]);
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
    fn subModule1() {
        let val = json!(["/hello", [0, 0, 0]]);
        let got = SubModule::from_value(&val);
        assert!(got.is_ok());
    }

    #[test]
    fn subModule2() {
        let val = json!(["this should fail because module name doesn't start with a slash", [0, 0, 0]]);
        let got = SubModule::from_value(&val);

        if got.is_ok() {
            panic!("{:?}", got);
        }
    }
}
