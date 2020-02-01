use serde::{Deserialize, Serialize};
use serde_repr::*;

// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// -- [ "line", [ 40, 8, 0, -4, 0 ] ]
// instance FromJSON Line where
//   parseJSON (Array v) = "Decode.Line.Array" <??
//     if V.length v == 2
//     then do
//       lineLabel <- parseJSON $ v V.! 0
//       when (lineLabel /= ("line" :: String)) $ fail "Not a line"
//       c5 <- parseJSON $ v V.! 1
//       return $ Line c5
//     else fail "Not a line"

pub struct Line {}

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

// fn main() {
//     //let rot = Rot::Rot0;

//     let deserialized: Rot = serde_json::from_str(&"0").unwrap();
//     println!("deserialized = {:?}", deserialized);

//     // let serialized = serde_json::to_string(&point).unwrap();
//     // println!("serialized = {}", serialized);
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn coord5() {
        let expect = Coord5 {
            x: 1,
            y: 2,
            r: Rot::Rot270,
            dx: 3,
            dy: 4,
        };
        let got: Coord5 = serde_json::from_str(&"[1, 2, 1, 3, 4]").unwrap();
        assert_eq!(expect, got);
    }
}
