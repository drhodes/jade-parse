use serde_json::Value;

use crate::common::*;
use crate::types::*;

impl Part {
    pub fn from_value(val: Value) -> E<Part> {
        if let x = Wire::from_value(&val)? {
            return Ok(Part::Wire(x));
        }
        if let x = Port::from_value(&val)? {
            return Ok(Part::Port(x));
        }
        if let x = Terminal::from_value(&val)? {
            return Ok(Part::Terminal(x));
        }
        if let x = SubModule::from_value(&val)? {
            return Ok(Part::SubModule(x));
        }
        if let x = Jumper::from_value(&val)? {
            return Ok(Part::Jumper(x));
        }
        if let x = Text::from_value(&val)? {
            return Ok(Part::Text(x));
        }
        bailfmt!("Part::from_value finds unknown part: {:?}", val)
    }
}

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::json;

//     #[test]
//     fn part1() {
//         let val = json!(["part", [0, 0, 0]]);
//         let got = Part::from_value(val);
//         assert!(got.is_ok());
//     }
// }
