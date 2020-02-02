use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::error::Error;

pub type E<T> = Result<T, Box<dyn Error>>;

pub fn err<T>(msg: &str) -> Result<T, Box<dyn Error>> {
    Err(From::from(msg.to_string()))
}

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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Coord3 {
    pub x: u32,
    pub y: u32,
    pub r: Rot,
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
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum Direction {
    In = 0,
    Out = 1,
    InOut = 2,
}

// ----------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct Symbol(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
