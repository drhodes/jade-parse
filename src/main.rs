use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;
use std::error::Error;

#[derive(Serialize, Debug, PartialEq)]
pub struct Line {
    coord: Coord5,
}

fn err<T>(msg: &str) -> Result<T, Box<dyn Error>> {
    Err(From::from(msg.to_string()))
}

pub fn line_des(val: Value) -> Result<Line, Box<dyn Error>> {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Coord5 {
    x: u32,
    y: u32,
    r: Rot,
    dx: u32,
    dy: u32,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum Rot {
    Rot0 = 0,
    Rot270 = 1,
    Rot180 = 2,
    Rot90 = 3,
    FlipX = 4,
    TransposeNeg = 5,
    FlipY = 6,
    TransposePos = 7,
}

fn main() {
    //let rot = Rot::Rot0;

    let deserialized: Rot = serde_json::from_str(&"0").unwrap();
    println!("deserialized = {:?}", deserialized);

    // let serialized = serde_json::to_string(&point).unwrap();
    // println!("serialized = {}", serialized);
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let got = line_des(v).unwrap();
        assert_eq!(expect, got);
    }

    #[test]
    fn line2() {
        let expect = Line { coord: Coord5 { x: 1, y: 2, r: Rot::Rot270, dx: 3, dy: 4 } };
        let s = r#"["line", [1, 2, 1, 3, 4]]"#;
        let v: Value = serde_json::from_str(s).unwrap();
        let got = line_des(v).unwrap();
        assert_eq!(expect, got);
    }
}
