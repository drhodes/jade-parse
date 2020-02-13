use serde_json::Value;

use crate::common::*;
use crate::types::*;

impl IconPart {
    pub fn from_value(val: &Value) -> E<IconPart> {
        if let Ok(x) = Line::from_value(val) {
            return Ok(IconPart::Line(x));
        }
        if let Ok(x) = Terminal::from_value(val) {
            return Ok(IconPart::Terminal(x));
        }
        if let Ok(x) = Text::from_value(val) {
            return Ok(IconPart::Text(x));
        }
        if let Ok(x) = Circle::from_value(val) {
            return Ok(IconPart::Circle(x));
        }
        bailfmt!("IconPart::from_value finds unknown iconPart: {:?}", val)
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
    fn iconPart1() {
        let val = json!( ["terminal", [32,0,4],{"name":"Ci"}] );
        let got = Terminal::from_value(&val);
        if !got.is_ok() {
            panic!("{:?}", got);
        }
    }
}
