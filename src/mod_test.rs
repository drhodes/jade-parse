use crate::common::*;
use crate::sig;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

const IDENT: &str = "[a-zA-Z_][a-zA-Z_0-9]*";
const SPACE: &str = r#"[\s]*"#;
const NUMBER: &str = r#"[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?"#;

// should probably use a grammar for this but the parsing story for
// rust at the moment is nascent but auspicious.

fn is_ident(s: &str) -> bool {
    let pat_str = format!("^{}$", IDENT);
    match regex::Regex::new(&pat_str).unwrap().find(s) {
        Some(_) => true,
        _ => false,
    }
}

impl ModTest {
    // pub fn default(test_str: &str) -> ModTest {
    //     ModTest { power: vec![],
    //               thresholds: None,
    //               groups: HashMap::new(),
    //               outputs: None,
    //               mode: None,
    //               cycle_line: None,
    //               test_lines: vec![],
    //               plot_def: vec![],
    //               plot_styles: vec![] }
    // }

    fn parse_power(s: &str) -> E<Vec<Power>> {
        if !s.starts_with(".power") {
            return bail!("not power line: todo improve this message");
        }
        // pattern (<ident> <space>* <equals> <space>* <num>)+

        let pat_str = format!("({}){}{}{}({})", IDENT, SPACE, "=", SPACE, NUMBER);
        let pat = regex::Regex::new(&pat_str).unwrap();
        let caps = pat.captures_iter(s);

        let mut powers = vec![];

        for cap in caps {
            match (cap.get(1), cap.get(2)) {
                (Some(name), Some(volts)) => {
                    powers.push(Power { name: name.as_str().to_string(),
                                        volts: volts.as_str().parse::<f64>().unwrap() });
                }
                _ => {}
            }
        }
        Ok(powers)
    }

    fn parse_thresholds(s: &str) -> E<Thresholds> {
        if !s.starts_with(".thresholds") {
            return bail!("not threshhold line: todo improve this message");
        }

        let f = |vxx: &str| {
            let pat_str = format!("{}{}={}({})", vxx, SPACE, SPACE, NUMBER);
            let pat = regex::Regex::new(&pat_str).unwrap();
            let caps = match pat.captures(s) {
                Some(caps) => caps,
                _ => return bailfmt!("No {} found in threshold line", vxx),
            };
            match caps.get(1) {
                Some(volts) => {
                    println!("{:?}, {:?}", caps, volts);
                    Ok(volts.as_str().parse::<f64>().unwrap())
                }
                None => bailfmt!("No {} found in threshold line", vxx),
            }
        };

        let voh = f("Voh")?;
        let vol = f("Vol")?;
        let vih = f("Vih")?;
        let vil = f("Vil")?;

        Ok(Thresholds { voh, vol, vih, vil })
    }

    fn parse_one_group(line: &str) -> E<(String, Vec<Sig>)> {
        // Jade does not allow spaces these signal names.
        //.group outputs BFN[3:0] A[31:0] B[31:0]
        let directive = ".group";
        if !line.starts_with(directive) {
            bailfmt!("not a {:?} directive", directive)?;
        }

        let line: &str = &line[directive.len()..].trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        // the first part should a identifier.
        let group_name = {
            if !is_ident(parts[0]) {
                return bailfmt!(".group requires an a1pha_9umeric name first, got: {}: ", parts[0]);
            }
            parts[0]
        };

        let mut sigs = vec![];
        for part in parts[1..].iter() {
            match sig::parse_sig(part) {
                Some(sig) => sigs.push(sig),
                None => return bailfmt!(".group contains a bad signal name: {}", part),
            }
        }
        Ok((group_name.to_string(), sigs))
    }

    fn parse_mode(line: &str) -> E<Mode> {
        if !line.starts_with(".mode") {
            bail!("not a mode directive")?;
        }
        let line: &str = &line[".mode".len()..].trim();

        match line {
            "gate" => Ok(Mode::Gate),
            "device" => Ok(Mode::Device),
            _ => bailfmt!("Unknown mode: {}", line),
        }
    }

    fn parse_cycle_line(s: &str) -> Option<CycleLine> {
        // .cycle assert inputs tran 99n sample outputs tran 1n

        todo!();
    }

    fn parse_test_lines(s: &str) -> Vec<TestLine> {
        todo!();
    }

    fn parse_plot_def(s: &str) -> Vec<PlotDef> {
        todo!();
    }

    fn parse_plot_styles(s: &str) -> Vec<PlotStyle> {
        todo!();
    }

    fn parse_plot_style(s: &str) -> Option<PlotStyle> {
        todo!();
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
    fn parse_mode1() {
        let got = ModTest::parse_mode(".mode    gate");
        let expect = Ok(Mode::Gate);
        assert_eq!(got, expect);
    }

    #[test]
    fn parse_mode2() {
        let got = ModTest::parse_mode(".mode device");
        let expect = Ok(Mode::Device);
        assert_eq!(got, expect);
    }

    #[test]
    fn parse_mode3() {
        match ModTest::parse_mode(".mode dvice") {
            Err(b) => {
                let msg = format!("Unknown mode: {}", "dvice");
                assert_eq!(b.msg, msg);
            }
            _ => panic!("this test should fail"),
        }
    }

    #[test]
    fn parse_one_group1() {
        let got = ModTest::parse_one_group(".group inputs BFN[3:0] A[31:0] B[31:0]");
        let expect = Ok(("inputs".to_string(),
                         vec![sig::parse_sig("BFN[3:0]").unwrap(),
                              sig::parse_sig("A[31:0]").unwrap(),
                              sig::parse_sig("B[31:0]").unwrap()]));
        assert_eq!(got, expect);
    }

    #[test]
    fn parse_threshold1() {
        let got = ModTest::parse_thresholds(".thresholds Vol=0 Vil=0.1 Vih=0.9 Voh=1");
        let expect = Ok(Thresholds { vol: 0.0, vil: 0.1, vih: 0.9, voh: 1.0 });
        assert_eq!(got, expect);
    }

    #[test]
    fn parse_threshold2() {
        // missing Voh
        match ModTest::parse_thresholds(".thresholds Vol=0.0 Vil=0.1 Vih=0.9") {
            Err(b) => {
                assert_eq!(b.msg, "No Voh found in threshold line");
            }
            _ => panic!("Expecting error got good value"),
        }
    }

    #[test]
    fn parse_power2() {
        let got = ModTest::parse_power(".power Vdd=1.0");
        let expect = vec![Power { name: "Vdd".to_string(), volts: 1.0f64 }];
        assert_eq!(got, Ok(expect));
    }

    #[test]
    fn parse_power3() {
        let got = ModTest::parse_power(".power Vdd=1.0 Foo=1.234");
        let expect = vec![Power { name: "Vdd".to_string(), volts: 1.0f64 },
                          Power { name: "Foo".to_string(), volts: 1.234f64 }];
        assert_eq!(got, Ok(expect));
    }

    fn bool_test() {
        let bool_test_str = r#"
.power Vdd=1
.thresholds Vol=0 Vil=0.1 Vih=0.9 Voh=1

.group inputs BFN[3:0] A[31:0] B[31:0]
.group outputs Y[31:0]

.mode gate

.cycle assert inputs tran 99n sample outputs tran 1n

0000 11111111000000001111111100000000 11111111111111110000000000000000 LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL //  1: bfn=0b0000, a=0XFF00FF00, b=0XFFFF0000, y=0X00000000
0001 11111111000000001111111100000000 11111111111111110000000000000000 LLLLLLLLLLLLLLLLLLLLLLLLHHHHHHHH //  2: bfn=0b0001, a=0XFF00FF00, b=0XFFFF0000, y=0X000000FF
0010 11111111000000001111111100000000 11111111111111110000000000000000 LLLLLLLLLLLLLLLLHHHHHHHHLLLLLLLL //  3: bfn=0b0010, a=0XFF00FF00, b=0XFFFF0000, y=0X0000FF00
0011 11111111000000001111111100000000 11111111111111110000000000000000 LLLLLLLLLLLLLLLLHHHHHHHHHHHHHHHH //  4: bfn=0b0011, a=0XFF00FF00, b=0XFFFF0000, y=0X0000FFFF
0100 11111111000000001111111100000000 11111111111111110000000000000000 LLLLLLLLHHHHHHHHLLLLLLLLLLLLLLLL //  5: bfn=0b0100, a=0XFF00FF00, b=0XFFFF0000, y=0X00FF0000
0101 11111111000000001111111100000000 11111111111111110000000000000000 LLLLLLLLHHHHHHHHLLLLLLLLHHHHHHHH //  6: bfn=0b0101, a=0XFF00FF00, b=0XFFFF0000, y=0X00FF00FF
0110 11111111000000001111111100000000 11111111111111110000000000000000 LLLLLLLLHHHHHHHHHHHHHHHHLLLLLLLL //  7: bfn=0b0110, a=0XFF00FF00, b=0XFFFF0000, y=0X00FFFF00
0111 11111111000000001111111100000000 11111111111111110000000000000000 LLLLLLLLHHHHHHHHHHHHHHHHHHHHHHHH //  8: bfn=0b0111, a=0XFF00FF00, b=0XFFFF0000, y=0X00FFFFFF
1000 11111111000000001111111100000000 11111111111111110000000000000000 HHHHHHHHLLLLLLLLLLLLLLLLLLLLLLLL //  9: bfn=0b1000, a=0XFF00FF00, b=0XFFFF0000, y=0XFF000000
1001 11111111000000001111111100000000 11111111111111110000000000000000 HHHHHHHHLLLLLLLLLLLLLLLLHHHHHHHH // 10: bfn=0b1001, a=0XFF00FF00, b=0XFFFF0000, y=0XFF0000FF
1010 11111111000000001111111100000000 11111111111111110000000000000000 HHHHHHHHLLLLLLLLHHHHHHHHLLLLLLLL // 11: bfn=0b1010, a=0XFF00FF00, b=0XFFFF0000, y=0XFF00FF00
1011 11111111000000001111111100000000 11111111111111110000000000000000 HHHHHHHHLLLLLLLLHHHHHHHHHHHHHHHH // 12: bfn=0b1011, a=0XFF00FF00, b=0XFFFF0000, y=0XFF00FFFF
1100 11111111000000001111111100000000 11111111111111110000000000000000 HHHHHHHHHHHHHHHHLLLLLLLLLLLLLLLL // 13: bfn=0b1100, a=0XFF00FF00, b=0XFFFF0000, y=0XFFFF0000
1101 11111111000000001111111100000000 11111111111111110000000000000000 HHHHHHHHHHHHHHHHLLLLLLLLHHHHHHHH // 14: bfn=0b1101, a=0XFF00FF00, b=0XFFFF0000, y=0XFFFF00FF
1110 11111111000000001111111100000000 11111111111111110000000000000000 HHHHHHHHHHHHHHHHHHHHHHHHLLLLLLLL // 15: bfn=0b1110, a=0XFF00FF00, b=0XFFFF0000, y=0XFFFFFF00
1111 11111111000000001111111100000000 11111111111111110000000000000000 HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH // 16: bfn=0b1111, a=0XFF00FF00, b=0XFFFF0000, y=0XFFFFFFFF
      
.plot X(BFN[3:0])
.plot X(A[31:0])
.plot X(B[31:0])
.plot X(Y[31:0])
"#;
    }
}
