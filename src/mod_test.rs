use crate::sig;
use crate::types::*;
use std::path::Path;

const IDENT: &str = "[a-zA-Z_][a-zA-Z_0-9]*";
const SPACE: &str = r#"[\s]*"#;
const ONE_OR_MORE_SPACE: &str = r#"[\s]+"#;
const NUMBER: &str = r#"[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?"#;
const DURATION: &str = r#"([-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?)([unpfaUNF])"#;

fn is_ident(s: &str) -> bool {
    let pat_str = format!("^{}$", IDENT);
    match regex::Regex::new(&pat_str).unwrap().find(s) {
        Some(_) => true,
        _ => false,
    }
}

impl ModTest {
    pub fn from_str(test_str: &str) -> E<ModTest> {
        let mut power: Vec<Power> = vec![];
        let mut thresholds = None;
        let mut groups = Groups::new();
        let mut mode = None;
        let mut cycle_line = None;
        let mut test_lines = vec![];
        let mut plot_dirs = vec![];
        let mut plot_defs = vec![];

        for line in test_str.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue; // skip whitespace
            } else if line.starts_with("//") {
                continue; // TODO keep comments and line numbers 
            } else if line.starts_with(".power") {
                let mut xs = Self::parse_power(line)?;
                power.append(&mut xs);
            } else if line.starts_with(".group") {
                let (name, group) = Self::parse_one_group(line)?;
                groups.insert_signals(name, group);
            } else if line.starts_with(".thresholds") {
                thresholds = Some(Self::parse_thresholds(line)?);
            } else if line.starts_with(".mode") {
                mode = Some(Self::parse_mode(line)?);
            } else if line.starts_with(".cycle") {
                cycle_line = Some(Self::parse_cycle_line(line)?);
            } else if line.starts_with(".plotdef") {
                plot_defs.push(Self::parse_plot_def(line)?);
            } else if line.starts_with(".plot ") {
                plot_dirs.push(Self::parse_plot_directive(line)?);
            } else {
                // this line is either junk or a test vector
                let tv = Self::parse_test_line(line)?;
                test_lines.push(tv)
            }
        }

        Ok(ModTest { power, thresholds, groups, mode, cycle_line, test_lines, plot_defs, plot_dirs })
    }

    pub fn from_file(p: &Path) -> E<ModTest> {
        match std::fs::read_to_string(p) {
            Ok(s) => Self::from_str(&s),
            Err(msg) => bailfmt!("Can't open test: {}", msg),
        }
    }

    fn parse_power(s: &str) -> E<Vec<Power>> {
        if !s.starts_with(".power") {
            return bail!("not power line: todo improve this message");
        }
        // pattern (<ident> <space>* <equals> <space>* <num>)+

        let pat_str = format!("({}){}={}({})", IDENT, SPACE, SPACE, NUMBER);
        let pat = regex::Regex::new(&pat_str).unwrap();
        let caps = pat.captures_iter(s);

        let mut powers = vec![];

        for cap in caps {
            match (cap.get(1), cap.get(2)) {
                (Some(name), Some(volts)) => {
                    powers.push(Power { name: name.as_str().to_string(),
                                        volts: volts.as_str().parse::<f64>().unwrap() });
                }
                // TODO this should return a syntax error.
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

    fn consume_action(line: &str) -> (E<Action>, &str) {
        let line = line.trim();
        // these should be static.
        let assert_pattern = format!("assert{}({})", ONE_OR_MORE_SPACE, IDENT);
        let deassert_pattern = format!("deassert{}({})", ONE_OR_MORE_SPACE, IDENT);
        let sample_pattern = format!("sample{}({})", ONE_OR_MORE_SPACE, IDENT);
        let tran_pattern = format!("tran{}{}", ONE_OR_MORE_SPACE, DURATION);

        if let Some(cap) = regex::Regex::new(&assert_pattern).unwrap().captures(line) {
            let span = cap.get(0).unwrap();
            if span.start() == 0 {
                let groupname = cap.get(1).unwrap().as_str().to_owned();
                return (Ok(Action::Assert(groupname)), &line[span.end()..]);
            }
        }

        if let Some(cap) = regex::Regex::new(&deassert_pattern).unwrap().captures(line) {
            let span = cap.get(0).unwrap();
            if span.start() == 0 {
                let groupname = cap.get(1).unwrap().as_str().to_owned();
                return (Ok(Action::Deassert(groupname)), &line[span.end()..]);
            }
        }

        if let Some(cap) = regex::Regex::new(&sample_pattern).unwrap().captures(line) {
            let span = cap.get(0).unwrap();
            if span.start() == 0 {
                let groupname = cap.get(1).unwrap().as_str().to_owned();
                return (Ok(Action::Sample(groupname)), &line[span.end()..]);
            }
        }

        if let Some(cap) = regex::Regex::new(&tran_pattern).unwrap().captures(line) {
            let span = cap.get(0).unwrap();
            if span.start() == 0 {
                match (cap.get(1), cap.get(3)) {
                    (Some(num), Some(unit)) => {
                        let n = num.as_str().parse::<f64>().unwrap();
                        const MSG: &'static str = "Unknown duration unit in .cycle: {:?}";
                        let action = Action::Tran(match unit.as_str() {
                                                      "u" | "U" => Duration::MicroSecond(n),
                                                      "n" | "N" => Duration::NanoSecond(n),
                                                      "p" | "P" => Duration::PicoSecond(n),
                                                      "f" | "F" => Duration::FemptoSecond(n),
                                                      "a" | "A" => Duration::AttoSecond(n),
                                                      x => {
                                                          return (bailfmt!("{} {}", MSG, x), "");
                                                      }
                                                  });
                        return (Ok(action), &line[span.end()..]);
                    }
                    _ => return (bailfmt!("Malformed tran in .cycle {:?}", span), ""),
                }
            }
        }

        // OK, this is involved. Need to split on =, take the left
        // side, trim and see if it parses as a sig.

        let parts: Vec<&str> = line.split("=").collect();
        if parts.len() > 1 {
            match sig::parse_sig(parts[0].trim()) {
                Some(sig) => {
                    // grab the right hand side and parse an identifier
                    let p = format!(r#"[\s]*({})"#, NUMBER);
                    if let Some(cap) = regex::Regex::new(&p).unwrap().captures(parts[1]) {
                        let span = cap.get(0).unwrap();
                        if span.start() == 0 {
                            // calculate num chars consumed.
                            // the + 1 in the middle counts the equal sign
                            let n = parts[0].len() + 1 + span.end();

                            let number = cap.get(1).unwrap().as_str().parse::<f64>().unwrap();
                            return (Ok(Action::SetSignal(sig, number)), &line[n..]);
                        }
                    }
                }
                None => return (bailfmt!("bad signal found: {:?}", line), ""),
            }
        } else {
            return (bailfmt!("unknown action in .cycle: {:?}", line), "");
        }
        return (bailfmt!("What's going on here?: {:?}", line), "");
    }

    fn parse_cycle_line(line: &str) -> E<CycleLine> {
        // .cycle assert inputs tran 99n sample outputs tran 1n
        if !line.starts_with(".cycle") {
            bail!("not a cycle directive")
        } else {
            let mut line = &line[6..];
            let mut actions: Vec<Action> = vec![];
            while !line.is_empty() {
                match Self::consume_action(line) {
                    (Ok(action), "") => {
                        actions.push(action);
                        break;
                    }
                    (Ok(action), rest) => {
                        actions.push(action);
                        line = rest.trim();
                    }
                    (berr, _) => return bail!(berr, "Couldn't parse .cycle directive"),
                }
            }
            Ok(CycleLine(actions))
        }
    }

    fn parse_test_line(s: &str) -> E<TestLine> {
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

    fn parse_plot_directive(line: &str) -> E<PlotDirective> {
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
                (x, y) => bailfmt!("unhandled case in parse_plot_directive ({:?}. {:?}", x, y),
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
    fn parse_cycle_line_1() {
        let line = ".cycle assert inputs";
        match ModTest::parse_cycle_line(line) {
            Ok(CycleLine(xs)) => assert_eq!(xs, vec!(Action::Assert("inputs".to_string()))),
            Err(berr) => panic!(berr),
        }
    }

    #[test]
    fn parse_cycle_line_2() {
        //let line = ".cycle assert inputs tran 99n sample outputs tran 1n";

        let line = ".cycle tran 99.5n";
        match ModTest::parse_cycle_line(line) {
            Ok(CycleLine(xs)) => assert_eq!(xs, vec!(Action::Tran(Duration::NanoSecond(99.5)))),
            Err(berr) => panic!("{:?}", berr),
        }
    }

    #[test]
    fn parse_cycle_line_3() {
        let line = ".cycle assert inputs tran 99n sample outputs tran 1n";
        match ModTest::parse_cycle_line(line) {
            Ok(CycleLine(xs)) => assert_eq!(xs,
                                            vec!(Action::Assert("inputs".to_string()),
                                                 Action::Tran(Duration::NanoSecond(99.0)),
                                                 Action::Sample("outputs".to_string()),
                                                 Action::Tran(Duration::NanoSecond(1.0)))),
            Err(berr) => panic!("{:?}", berr),
        }
    }

    #[test]
    fn parse_cycle_line_4() {
        let line = ".cycle tran 1n assert A sample B assert C tran 2n";
        match ModTest::parse_cycle_line(line) {
            Ok(CycleLine(xs)) => assert_eq!(xs,
                                            vec!(Action::Tran(Duration::NanoSecond(1.0)),
                                                 Action::Assert("A".to_string()),
                                                 Action::Sample("B".to_string()),
                                                 Action::Assert("C".to_string()),
                                                 Action::Tran(Duration::NanoSecond(2.0)))),

            Err(berr) => panic!("{:?}", berr),
        }
    }

    #[test]
    fn parse_cycle_line_5() {
        let line = ".cycle CLK=1";
        match ModTest::parse_cycle_line(line) {
            Ok(CycleLine(xs)) => {
                let sig = sig::parse_sig("CLK").unwrap();
                assert_eq!(xs, vec!(Action::SetSignal(sig, 1.0)));
            }
            Err(berr) => panic!("{:?}", berr),
        }
    }

    #[test]
    fn from_file_1() {
        match ModTest::from_file(Path::new("./test-data/modtests/bool.test")) {
            Ok(x) => {}
            Err(msg) => panic!("{:?}", msg),
        }
    }

    #[test]
    fn parse_plot_directive1() {
        let got = ModTest::parse_plot_directive(".plot X(Y[31:0])");
        let sig = sig::parse_sig("Y[31:0]").unwrap();
        let expect = PlotDirective::HexStyle(sig);
        assert_eq!(got, Ok(expect));
    }

    #[test]
    fn parse_plot_directive2() {
        let got = ModTest::parse_plot_directive(".plot clk");
        let sig = sig::parse_sig("clk").unwrap();
        let expect = PlotDirective::SimplePlot(sig);
        assert_eq!(got, Ok(expect));
    }

    #[test]
    fn parse_plot_def() {
        let got = ModTest::parse_plot_def(".plotdef op ADD SUB MUL");
        let expect =
            PlotDef { name: "op".to_string(), tags: vec!["ADD".to_string(), "SUB".to_string(), "MUL".to_string()] };
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
        let expect =
            vec![Power { name: "Vdd".to_string(), volts: 1.0f64 }, Power { name: "Foo".to_string(), volts: 1.234f64 }];
        assert_eq!(got, Ok(expect));
    }
}
