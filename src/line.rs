use crate::types::*;
use serde_json::Value;

impl Line {
    pub fn from_value(val: Value) -> E<Line> {
        match val {
            Value::Array(v) => {
                if v.len() != 2 {
                    return err("line expects array with two args");
                }
                if v[0] != Value::String("line".to_string()) {
                    return err("Line expects first array element to be the string 'line'");
                }
                let c: Coord5 = serde_json::from_value(v[1].clone())?;
                Ok(Line { coord: c })
            }
            _ => err("line expects array with two args"),
        }
    }
}

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::types::*;

    #[test]
    fn coord5() {
        let expect = Coord5 { x: 1, y: 2, r: Rot::Rot270, dx: 3, dy: 4 };
        let got: Coord5 = serde_json::from_str(&"[1, 2, 1, 3, 4]").unwrap();
        assert_eq!(expect, got);
    }

    #[test]
    fn line1() {
        let expect = Line { coord: Coord5 { x: 1, y: 2, r: Rot::Rot270, dx: 3, dy: 4 } };
        let s = r#"["line", [1, 2, 1, 3, 4]]"#;
        let v: Value = serde_json::from_str(s).unwrap();
        let got: Line = Line::from_value(v).unwrap();
        assert_eq!(expect, got);
    }
}
