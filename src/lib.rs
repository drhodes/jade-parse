#![feature(box_patterns)]
#![allow(warnings)]
#![recursion_limit = "256"]

#[macro_use]
pub mod bail;
pub mod common;

// schematic
pub mod dir;
pub mod jumper;
pub mod line;
pub mod part;
pub mod sig;
pub mod signal;
pub mod submodule;
pub mod types;
pub mod wire;

// icon
pub mod circle;
pub mod port;
pub mod terminal;
pub mod text;

pub mod test_mod;
