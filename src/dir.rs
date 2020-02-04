use crate::types::*;
use serde_json::Value;

impl Direction {
    pub fn from_value(val: Value) -> E<Direction> {
        if let Value::String(dir) = val {
            match dir.as_str() {
                "in" => return Ok(In),
                "out" => return Ok(Out),
                "inout" => return Ok(InOut),
                _ => {
                    return err(format!("Got a bad signal direction: {:?}", dir).as_str());
                }
            }
        } else {
            return err("not a signal direction");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::types::*;
    use serde_json::json;

    #[test]
    fn dir1() {
        let expect = In;
        let got = Direction::from_value(json!("in")).unwrap();
        assert_eq!(expect, got);
    }

    #[test]
    fn dir2() {
        let got: E<Direction> = Direction::from_value(json!("garbage-in"));
        assert!(got.is_err());
    }
}
