#![feature(box_patterns)]
#![allow(warnings)]
#![recursion_limit = "256"]

#[macro_use]
pub mod bail;
pub mod common;

//pub mod project;
pub mod mod_test;

// schematic
pub mod dir;
pub mod jumper;
pub mod line;
pub mod part;
pub mod schematic;
pub mod sig;
pub mod signal;
pub mod submodule;
pub mod types;
pub mod wire;

// icon
pub mod circle;
pub mod icon;
pub mod icon_part;
pub mod port;
pub mod terminal;
pub mod text;

// test aspect
pub mod groups;
pub mod test_mod;
