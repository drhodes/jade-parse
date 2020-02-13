pub use crate::bail::*;

use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::error::Error;
use std::fmt;
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
    pub icon: u8,
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

//-----------------------------------------------------------------------------
//
// JADE TESTS
//

pub struct Power {
    pub vdd: f64,
}

pub struct Thresholds {
    pub vol: f64,
    pub vil: f64,
    pub vih: f64,
    pub voh: f64,
}

pub struct Inputs(Vec<Sig>);
pub struct Outputs(Vec<Sig>);

pub enum Mode {
    Device,
    Gate,
}

pub enum Duration {
    NanoSecond(f64),
    MilliSecond(f64),
    //PicoSecond(f64),
}

pub enum Action {
    Assert(String),
    Deassert(String),
    Tran(Duration),
    SetSignal(Sig, f64),
}

pub struct CycleLine(Vec<Action>);

pub enum BinVal {
    L,
    H,
    Z,
}
pub struct TestLine {
    pub bin_vals: Vec<BinVal>,
    pub comment: Option<String>,
}

pub struct PlotDef {
    sig: Sig,
}

pub enum PlotStyle {
    BinStyle(Sig),
    HexStyle(Sig),
    DecStyle(Sig),
    SimplePlot(Sig),
    PlotDefStyle(String, Sig),
}

pub struct Test {
    pub power: Option<Power>,
    pub thresholds: Option<Thresholds>,
    pub inputs: Option<Inputs>,
    pub outputs: Option<Outputs>,
    pub mode: Option<Mode>,
    pub cycle_line: Option<CycleLine>,
    pub test_lines: Vec<TestLine>,
    pub plot_def: Vec<PlotDef>,
    pub plot_styles: Vec<PlotStyle>,
}
