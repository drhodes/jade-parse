pub use crate::bail::*;

use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;

// -----------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Coord5 {
    pub x: u32,
    pub y: u32,
    pub r: Rot,
    pub dx: u32,
    pub dy: u32,
}

// -----------------------------------------------------------------------------
#[derive(Deserialize, Debug, PartialEq)]
pub struct Coord3 {
    pub x: u32,
    pub y: u32,
    pub r: Rot,
}

// -----------------------------------------------------------------------------
// "schematic":[
//               ["/gates/buffer", [-16,-32,0]],
//               ["wire",[-16,-32,0,-8,0],{"signal":"A"}],
//               ["wire",[16,-32,0,8,0],{"signal":"vout"}]]
//

// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct Wire {
    pub coord5: Coord5,
    pub signal: Option<Signal>,
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
