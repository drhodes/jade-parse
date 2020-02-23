pub use crate::bail::*;

use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path;

// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Project {
    pub modules: Vec<Module>,
    pub filename: path::Path,
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub properties: u8,
    pub schematic: Schematic,
    pub icon: Option<Icon>,
    pub test: u8,
}

pub struct Properties;

#[derive(Debug)]
pub struct Schematic {
    pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Icon {
    pub parts: Vec<IconPart>,
}

#[derive(Debug, PartialEq)]
pub enum Part {
    Port(Port),
    Wire(Wire),
    Jumper(Jumper),
    Terminal(Terminal),
    Text(Text),
    SubModule(SubModule),
}

#[derive(Debug, PartialEq)]
pub enum IconPart {
    Line(Line),
    Terminal(Terminal),
    Text(Text),
    Circle(Circle),
    // Property,
    // Arc,
}

// #[derive(Debug, PartialEq)]
// pub enum Part {
//     Port(Port),
//     Wire(Wire),
//     Jumper(Jumper),
//     Terminal(Terminal),
//     Text(Text),
//     // match submodule last.
//     SubModule(SubModule),
// }

// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct SubModule {
    pub name: String,
    pub coord3: Coord3,
}

// -----------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Coord5 {
    pub x: i32,
    pub y: i32,
    pub r: Rot,
    pub dx: i32,
    pub dy: i32,
}

// -----------------------------------------------------------------------------
#[derive(Deserialize, Debug, PartialEq)]
pub struct Coord3 {
    pub x: i32,
    pub y: i32,
    pub r: Rot,
}

// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct Wire {
    pub coord5: Coord5,
    pub signal: Option<Signal>,
}

// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct Jumper {
    pub coord3: Coord3,
}

// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Default)]
pub struct Signal {
    pub sig: Option<Sig>,
    pub width: Option<u64>,
    pub direction: Option<Direction>,
    // pub net_signal: Option<Box<Signal>>, not sure what this is about.
}

// ----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct Line {
    pub coord: Coord5,
}

// -----------------------------------------------------------------------------
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Rot {
    Rot0 = 0,
    Rot270 = 1,
    Rot180 = 2,
    Rot90 = 3,
    FlipX = 4,
    TransposeNeg = 5,
    FlipY = 6,
    TransposePos = 7,
}
pub use Rot::*;

// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub enum Direction {
    In,
    Out,
    InOut,
}
pub use Direction::*;

// ----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct Symbol(pub String);

#[derive(Deserialize, Debug, PartialEq)]
pub enum Sig {
    SigSimple(String),
    SigIndex(String, i32),
    SigHash(String, i32),
    SigRange(String, i32, i32),
    SigRangeStep(String, i32, i32, i32),
    SigQuote(i32, i32),
    SigConcat(Vec<Sig>),
}
pub use Sig::*;

#[derive(Debug, PartialEq)]
pub struct Circle {
    pub x: u32,
    pub y: u32,
    pub rot: Rot,
    pub radius: f64,
}

#[derive(Debug, PartialEq)]
pub struct Text {
    pub coord3: Coord3,
    pub text: String,
    // TODO: "align": "center", add this field
    pub font: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Port {
    pub coord3: Coord3,
    pub signal: Option<Signal>,
}

#[derive(Debug, PartialEq)]
pub struct Terminal {
    pub coord3: Coord3,
    pub sig: Sig,
}

// -----------------------------------------------------------------------------
// JADE TESTS
//

#[derive(Debug, PartialEq)]
pub struct Power {
    pub name: String,
    pub volts: f64,
}

#[derive(Debug, PartialEq)]
pub struct Thresholds {
    pub vol: f64,
    pub vil: f64,
    pub vih: f64,
    pub voh: f64,
}

#[derive(Debug, PartialEq)]
pub struct Groups {
    pub sig_set: HashMap<GroupName, Vec<Sig>>,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Device,
    Gate,
}

#[derive(Debug, PartialEq)]
pub enum Duration {
    MicroSecond(f64),
    NanoSecond(f64),
    PicoSecond(f64),
    FemptoSecond(f64),
    AttoSecond(f64),
}

// from the jade tutorial:
// Tests are sequences of lines supplying test values; .cycle specifies
// the sequence of actions that will be performed for each test.  Available
// actions are
//    assert group -- set values for signals in group with H,L test values
//    deassert group -- stop setting values for signals in group with H,L test values
//    sample group -- check values of signals in group with 0,1 test values
//    tran time -- run simulation for specified time interval
//    signal=val -- set signal to specified value

type GroupName = String;

#[derive(Debug, PartialEq)]
pub enum Action {
    Assert(GroupName),
    Deassert(GroupName),
    Sample(GroupName),
    Tran(Duration),
    SetSignal(Sig, f64),
}

#[derive(Debug, PartialEq)]
pub struct CycleLine(pub Vec<Action>);

#[derive(Debug, PartialEq)]
pub enum BinVal {
    L,        // binary low
    H,        // binary high
    X,        // an unknown or illegal logic value
    Z,        // not driven, aka "high impedence"
    DontCare, // Don't Care
}
pub use BinVal::*;

#[derive(Debug, PartialEq)]
pub struct TestLine {
    pub bin_vals: Vec<BinVal>,
    pub comment: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum PlotDirective {
    BinStyle(Sig),
    HexStyle(Sig),
    DecStyle(Sig),
    SimplePlot(Sig),
    PlotDefStyle(String, Sig),
}

#[derive(Debug, PartialEq)]
pub struct PlotDef {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct ModTest {
    pub power: Vec<Power>,
    pub thresholds: Option<Thresholds>,
    pub groups: Groups,
    pub mode: Option<Mode>,
    pub cycle_line: Option<CycleLine>,
    pub test_lines: Vec<TestLine>,
    pub plot_dirs: Vec<PlotDirective>,
    pub plot_defs: Vec<PlotDef>,
}
