use crate::common::*;
use crate::sig;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

const IDENT: &str = "[a-zA-Z_][a-zA-Z_0-9]*";
const SPACE: &str = r#"[\s]*"#;
const NUMBER: &str = r#"[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?"#;

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
                Some(volts) => Ok(volts.as_str().parse::<f64>().unwrap()),
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
                let msg = ".group requires an a1pha_9umeric name first, got";
                return bailfmt!("{}: {:?}", msg, parts[0]);
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

    fn parse_test_line(mut s: &str) -> E<TestLine> {
        let parse_chars = |cs: &str| -> E<Vec<BinVal>> {
            let mut binvals = vec![];
            for c in cs.chars() {
                if c.is_whitespace() {
                    continue;
                }
                let msg = "Bad value in test line, expecting L, H, X, Z or -, got: ";
                binvals.push(match c {
                                 'L' => L, // binary low
                                 'H' => H, // binary high
                                 '0' => L, // binary low
                                 '1' => H, // binary high
                                 'X' => X, // an unknown or illegal logic value
                                 'Z' => Z, // not driven, aka "high impedence"
                                 '-' => DontCare,
                                 _ => {
                                     return bailfmt!("{}{:?}", msg, c);
                                 }
                             });
            }
            return Ok(binvals);
        };
        if let Some(idx) = s.find("//") {
            // does the line contain a "//" comment?
            let data = &s[0..idx];
            let comment = &s[idx + 2..];
            let bin_vals = parse_chars(data.trim())?;
            return Ok(TestLine { bin_vals, comment: Some(comment.to_string()) });
        } else {
            // no comment found.
            let bin_vals = parse_chars(s)?;
            return Ok(TestLine { bin_vals, comment: None });
        }
    }

    fn parse_plot_def(line: &str) -> E<PlotDef> {
        // .plotdef reg R0 R1 ... R31

        if !line.starts_with(".plotdef ") {
            bail!("Not a plot definition directive")
        } else {
            let line: &str = line[8..].trim();
            let mut tags = vec![];
            let mut name = String::new();

            for tag in line.split_whitespace() {
                // the first word is the plotdef name.
                if name.is_empty() {
                    if is_ident(&tag) {
                        name = tag.to_string();
                    } else {
                        let msg = ".plotdef directive identifier must be an alphanumeric, got";
                        return bailfmt!("{}: '{}'", msg, tag);
                    }
                } else {
                    tags.push(tag.to_string());
                }
            }
            Ok(PlotDef { name, tags })
        }
    }

    fn parse_plot_line(line: &str) -> E<PlotDirective> {
        if !line.starts_with(".plot ") {
            bail!("Not a plot directive")
        } else {
            let line: &str = line[5..].trim();
            // need to match something like this:  .plot X(Y[31:0])
            //                           or just:  .plot Y

            let pat_str = r#"([a-zA-Z_][a-zA-Z0-9_]*)(\(([^\)]+)\))?"#;
            let pat = regex::Regex::new(&pat_str).unwrap();
            let caps = match pat.captures(line) {
                Some(caps) => caps,
                _ => return bailfmt!("Bad signal found in .plot directive: {}", line),
            };
            match (caps.get(1), caps.get(3)) {
                (Some(ident), Some(sig_string)) => {
                    // let sig_string = sig_string[
                    let sig = Sig::from_str(sig_string.as_str());
                    let sig = bailif!(sig, "Bad signal name in .plot directive")?;

                    let ident = ident.as_str();
                    Ok(match ident {
                        "B" => PlotDirective::BinStyle(sig),
                        "X" => PlotDirective::HexStyle(sig),
                        "D" => PlotDirective::DecStyle(sig),
                        _ => PlotDirective::PlotDefStyle(ident.to_string(), sig),
                    })
                }
                (Some(sig_string), None) => {
                    let sig = Sig::from_str(sig_string.as_str())?;
                    Ok(PlotDirective::SimplePlot(sig))
                }
                (x, y) => bailfmt!("unhandled case in parse_plot_line ({:?}. {:?}", x, y),
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

    #[test]
    fn parse_plot_line1() {
        let got = ModTest::parse_plot_line(".plot X(Y[31:0])");
        let sig = sig::parse_sig("Y[31:0]").unwrap();
        let expect = PlotDirective::HexStyle(sig);
        assert_eq!(got, Ok(expect));
    }

    #[test]
    fn parse_plot_line2() {
        let got = ModTest::parse_plot_line(".plot clk");
        let sig = sig::parse_sig("clk").unwrap();
        let expect = PlotDirective::SimplePlot(sig);
        assert_eq!(got, Ok(expect));
    }

    #[test]
    fn parse_plot_def() {
        let got = ModTest::parse_plot_def(".plotdef op ADD SUB MUL");
        let expect = PlotDef { name: "op".to_string(),
                               tags: vec!["ADD".to_string(), "SUB".to_string(), "MUL".to_string()] };
        assert_eq!(got, Ok(expect));
    }

    #[test]
    fn parse_testline1() {
        let got = ModTest::parse_test_line("HHH LLL 110 --- // last three don't care");
        let bin_vals = vec![H, H, H, L, L, L, H, H, L, DontCare, DontCare, DontCare];
        let comment = Some(" last three don't care".to_string());
        let expect = Ok(TestLine { bin_vals, comment });
        assert_eq!(got, expect);
    }

    #[test]
    fn parse_testline2() {
        let got = ModTest::parse_test_line("HHH LLL 110 ---");
        let bin_vals = vec![H, H, H, L, L, L, H, H, L, DontCare, DontCare, DontCare];
        let comment = None;
        let expect = Ok(TestLine { bin_vals, comment });
        assert_eq!(got, expect);
    }

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
