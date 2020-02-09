use serde_json::Value;

use crate::common::*;
use crate::types::*;
use std::convert::From;

impl Text {
    pub fn from_value(val: &Value) -> E<Text> {
        let mut val_iter = bailif!(tagged_array("text", &val), "Text::from_value failes to decode")?;

        let coord3: Coord3 = match val_iter.next() {
            Some(c) => serde_json::from_value::<Coord3>(c.clone())?,
            None => {
                return bailfmt!("Text expects 2 elements, a location and properties object, got: {:?}", val);
            }
        };

        match val_iter.next() {
            Some(Value::Object(o)) => {
                // All Text objects should have a string object
                let text = if let Some(Value::String(text_val)) = o.get("text") {
                    text_val.to_string()
                } else {
                    return bail!("expected a string but didn't find anything");
                };

                let font = if let Some(Value::String(font_str)) = o.get("font") {
                    Some(font_str.to_string())
                } else {
                    None
                };

                return Ok(Text { coord3, text, font });
            }

            _ => {
                return bailfmt!("Text expects 2 elements, a location and properties object, got: {:?}", val);
            }
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
    fn text1() {
        let coord3 = Coord3 { x: 0, y: 0, r: Rot0 };
        let expect = Text { coord3: coord3,
                            text: "memories of green".to_string(),
                            font: Some("bladerunner".to_string()) };
        let val = json!(["text", [0,0,0], {"text": "memories of green", "font": "bladerunner"}]);
        let got: Text = Text::from_value(&val).unwrap();
        assert_eq!(expect, got);
    }

    #[test]
    fn text2() {
        let coord3 = Coord3 { x: 1, y: 2, r: Rot0 };
        let expect = Text { coord3: coord3, text: "memories of green".to_string(), font: None };
        let val = json!(["text", [1,2,0], {"text": "memories of green"}]);
        let got: Text = Text::from_value(&val).unwrap();
        assert_eq!(expect, got);
    }
}
