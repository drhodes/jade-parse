use crate::types::*;
use regex::Regex;
//use serde_json::Value;

// these should all be Result instead of Option for error messages.

pub fn sig_simple(input: &str) -> Option<Sig> {
    let pat = regex::Regex::new(r#"^[a-zA-Z_][a-zA-Z0-9_]*$"#).unwrap();
    if pat.is_match(input) { Some(SigSimple(input.to_string())) } else { None }
}

pub fn sig_hash(input: &str) -> Option<Sig> {
    let pat = regex::Regex::new(r#"(^[a-zA-Z_][a-zA-Z0-9_]*)#([0-9]+)$"#).unwrap();
    let caps = pat.captures(input)?;
    let sym = caps.get(1)?.as_str().to_string();
    let size = caps.get(2)?.as_str().parse::<i32>().unwrap();
    return Some(SigHash(sym, size));
}

pub fn sig_index(input: &str) -> Option<Sig> {
    let pat = regex::Regex::new(r#"(^[a-zA-Z_][a-zA-Z0-9_]*)\[([0-9]+)\]$"#).unwrap();
    let caps = pat.captures(input)?;
    let sym = caps.get(1)?.as_str().to_string();
    let idx = caps.get(2)?.as_str().parse::<i32>().unwrap();
    return Some(SigIndex(sym, idx));
}

pub fn sig_range(input: &str) -> Option<Sig> {
    let pat = regex::Regex::new(r#"(^[a-zA-Z_][a-zA-Z0-9_]*)\[([0-9]+):([0-9]+)\]$"#).unwrap();
    let caps = pat.captures(input)?;
    let sym = caps.get(1)?.as_str().to_string();
    let from = caps.get(2)?.as_str().parse::<i32>().unwrap();
    let to = caps.get(3)?.as_str().parse::<i32>().unwrap();
    Some(SigRange(sym, from, to))
}

pub fn sig_range_step(input: &str) -> Option<Sig> {
    let pat = regex::Regex::new(r#"(^[a-zA-Z_][a-zA-Z0-9_]*)\[([0-9]+):([0-9]+):([0-9]+)\]$"#).unwrap();
    let caps = pat.captures(input)?;
    let sym = caps.get(1)?.as_str().to_string();
    let from = caps.get(2)?.as_str().parse::<i32>().unwrap();
    let to = caps.get(3)?.as_str().parse::<i32>().unwrap();
    let step = caps.get(4)?.as_str().parse::<i32>().unwrap();
    Some(SigRangeStep(sym, from, to, step))
}

pub fn general_sig_quote(input: &str, pattern_string: &str, prefix: &str, radix: u32) -> Option<Sig> {
    let pat = Regex::new(pattern_string).unwrap();
    let caps = pat.captures(input)?;
    let numval = caps.get(1)?;
    let numbits = caps.get(2)?;
    let numval = i32::from_str_radix(numval.as_str().trim_start_matches(prefix), radix).unwrap();
    let numbits = numbits.as_str().parse::<i32>().unwrap();
    return Some(SigQuote(numval, numbits));
}

pub fn bin_sig_quote(input: &str) -> Option<Sig> {
    general_sig_quote(input, "^(0b[01]+)'([0-9])$", "0b", 2)
}

pub fn dec_sig_quote(input: &str) -> Option<Sig> {
    general_sig_quote(input, "^(0d[0-9]+)'([0-9])$", "0d", 10)
}

pub fn hex_sig_quote(input: &str) -> Option<Sig> {
    general_sig_quote(input, "^(0x[0-9A-Fa-f]+)'([0-9])$", "0x", 16)
}

pub fn implicit_dec_sig_quote(input: &str) -> Option<Sig> {
    general_sig_quote(input, "^([0-9]+)'([0-9])$", "", 10)
}

pub fn sig_quote(input: &str) -> Option<Sig> {
    for f in &[implicit_dec_sig_quote, bin_sig_quote, hex_sig_quote, dec_sig_quote] {
        let sig = f(input);
        if sig.is_some() {
            return sig;
        }
    }
    return None;
}

pub fn one_of_sig(input: &str) -> Option<Sig> {
    for f in &[sig_simple, sig_hash, sig_range, sig_range_step, sig_index, sig_quote] {
        let sig = f(input);
        if sig.is_some() {
            return sig;
        }
    }
    return None;
}

pub fn sig_concat(input: &str) -> Option<Sig> {
    if !input.contains(",") {
        // not a concatenation, maybe another signal type.
        return None;
    }
    let mut sigs = vec![];
    for sigtxt in input.split(",") {
        let sig = one_of_sig(sigtxt.trim())?;
        sigs.push(sig);
    }
    Some(SigConcat(sigs))
}

pub fn parse_sig(input: &str) -> Option<Sig> {
    let sig = one_of_sig(input);
    if sig.is_some() { sig } else { sig_concat(input) }
}

// impl Sig {
//     pub fn from_value(val: Value) -> E<Sig> {
//         if let Value::String(sig_string) = val -> E<Sig> {
//             if let Some(sig) = parse_sig(sig_string) {
//                 return Ok(sig);
//             } else {
//                 return err(format!("error parsing signal string: {:?}", sig_string));
//             }
//         } else {
//             return err("format!("Sig::from_valueexpected string, got: {:?}",
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::types::*;

    #[test]
    fn sig_simple1() {
        let expect = SigSimple("ABC123".to_string());
        let got = sig_simple("ABC123").unwrap();
        assert_eq!(expect, got);
    }
    #[test]
    fn sig_simple2() {
        let expect = SigSimple("_ABC123".to_string());
        let got = sig_simple("_ABC123").unwrap();
        assert_eq!(expect, got);
    }
    #[test]
    fn sig_simple3() {
        let got = sig_simple("#$%^9_ABC123");
        let expect = None;
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_hash1() {
        let got = sig_hash("A#32");
        let expect = Some(SigHash("A".to_string(), 32));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_hash2() {
        let got = sig_hash("asdf#32");
        let expect = Some(SigHash("asdf".to_string(), 32));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_hash3() {
        let got = sig_hash("asdf#00123");
        let expect = Some(SigHash("asdf".to_string(), 123));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_hash4() {
        let got = sig_hash("_asdf456#123");
        let expect = Some(SigHash("_asdf456".to_string(), 123));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_hash5() {
        let got = sig_hash("9_asdf456#123");
        let expect = None;
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_index2() {
        let got = sig_index("asdf[32]");
        let expect = Some(SigIndex("asdf".to_string(), 32));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_index3() {
        let got = sig_index("asdf[00123]");
        let expect = Some(SigIndex("asdf".to_string(), 123));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_index4() {
        let got = sig_index("_asdf456[123]");
        let expect = Some(SigIndex("_asdf456".to_string(), 123));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_index5() {
        let got = sig_index("9_asdf456[123]");
        let expect = None;
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_range2() {
        let got = sig_range("asdf[32:12]");
        let expect = Some(SigRange("asdf".to_string(), 32, 12));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_range3() {
        let got = sig_range("asdf[00123:56]");
        let expect = Some(SigRange("asdf".to_string(), 123, 56));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_range4() {
        let got = sig_range("_asdf456[123:0]");
        let expect = Some(SigRange("_asdf456".to_string(), 123, 0));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_range5() {
        let got = sig_range("9_asdf456[123:23]");
        let expect = None;
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_range_step2() {
        let got = sig_range_step("asdf[32:12:2]");
        let expect = Some(SigRangeStep("asdf".to_string(), 32, 12, 2));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_range_step3() {
        let got = sig_range_step("asdf[00123:56:4]");
        let expect = Some(SigRangeStep("asdf".to_string(), 123, 56, 4));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_range_step4() {
        let got = sig_range_step("_asdf456[123:0:5]");
        let expect = Some(SigRangeStep("_asdf456".to_string(), 123, 0, 5));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_range_step5() {
        let got = sig_range_step("9_asdf456[123:23:6]");
        let expect = None;
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_quote1() {
        let got = bin_sig_quote("0b1010'4");
        let expect = Some(SigQuote(10, 4));
        assert_eq!(got, expect);
    }
    #[test]
    fn sig_quote11() {
        let got = sig_quote("0b1010'4");
        let expect = Some(SigQuote(10, 4));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_quote2() {
        let got = hex_sig_quote("0xFF'4");
        let expect = Some(SigQuote(0xFF, 4));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_quote3() {
        let got = implicit_dec_sig_quote("10'4");
        let expect = Some(SigQuote(10, 4));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_quote4() {
        let got = sig_quote("10'4");
        let expect = Some(SigQuote(10, 4));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_quote5() {
        let got = sig_quote("0x12'4");
        let expect = Some(SigQuote(0x12, 4));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_concat1() {
        let got = sig_concat("0x12'4, 0x12'4");
        let expect = Some(SigConcat(vec![SigQuote(0x12, 4), SigQuote(0x12, 4)]));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_concat2() {
        let got = sig_concat("0x12'4, _asdf456[123:0:5]");
        let expect = Some(SigConcat(vec![SigQuote(0x12, 4), sig_range_step("_asdf456[123:0:5]").unwrap()]));
        assert_eq!(got, expect);
    }

    #[test]
    fn sig_concat3() {
        let got = one_of_sig("0x12'4, _asdf456[123:0:5]");
        let expect = None;
        assert_eq!(got, expect);
    }
}
