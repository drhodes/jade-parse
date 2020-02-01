use crate::types::*;
use regex;
// parse string into an array of symbols.  Canonicalize all text to lower case.
//  sig_list := sig[,sig]...
//  sig := symbol
//      := sig#count         replicate sig specified number of times
//      := sig[start:stop:step]   expands to sig[start],sig[start+step],...,sig[end]
//      := number'size       generate appropriate list of vdd, gnd to represent number

pub fn symbol(input: &str) -> Option<Sig> {
    let pat = regex::Regex::new(r#"^[a-zA-Z_][a-zA-Z0-9_]*$"#).unwrap();
    if pat.is_match(input) { Some(SigSimple(input.to_string())) } else { None }
}

pub fn sig_hash(input: &str) -> Option<Sig> {
    let pat = regex::Regex::new(r#"(^[a-zA-Z_][a-zA-Z0-9_]*)#([0-9]+)$"#).unwrap();
    let caps = pat.captures(input)?;
    let sym = caps.get(1);
    let size = caps.get(2);

    if sym.is_some() && size.is_some() {
        let sym = sym.unwrap().as_str().to_string();
        let size = size.unwrap().as_str().parse::<i32>().unwrap();
        return Some(SigHash(sym, size));
    } else {
        return None;
    }
}

pub fn sig_index(input: &str) -> Option<Sig> {
    let pat = regex::Regex::new(r#"(^[a-zA-Z_][a-zA-Z0-9_]*)\[([0-9]+)\]$"#).unwrap();
    let caps = pat.captures(input)?;
    let sym = caps.get(1);
    let idx = caps.get(2);

    if sym.is_some() && idx.is_some() {
        let sym = sym.unwrap().as_str().to_string();
        let size = idx.unwrap().as_str().parse::<i32>().unwrap();
        return Some(SigIndex(sym, size));
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::types::*;

    #[test]
    fn symbol1() {
        let expect = SigSimple("ABC123".to_string());
        let got = symbol("ABC123").unwrap();
        assert_eq!(expect, got);
    }
    #[test]
    fn symbol2() {
        let expect = SigSimple("_ABC123".to_string());
        let got = symbol("_ABC123").unwrap();
        assert_eq!(expect, got);
    }
    #[test]
    fn symbol3() {
        let got = symbol("#$%^9_ABC123");
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
}
