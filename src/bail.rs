use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;

pub type E<T> = Result<T, Bail>;

#[macro_export]
macro_rules! bail {
    ($msg:expr) => {
        Err(Bail { line: line!(),
                   col: column!(),
                   msg: $msg.to_string(),
                   file: file!().to_string(),
                   more: None })
    };
    ($ebail:expr, $msg:expr) => {
        Err(Bail { line: line!(),
                   col: column!(),
                   msg: $msg.to_string(),
                   file: file!().to_string(),
                   more: Some(Box::new($ebail.unwrap_err())) })
    };
}

#[macro_export]
macro_rules! bailfmt {
    ($msg:expr, $e:expr) => {
        bail!(format!($msg, $e));
    };
    ($msg:expr, $e1:expr, $e2:expr) => {
        bail!(format!($msg, $e1, $e2));
    };
}

#[macro_export]
macro_rules! bailif {
    ($ebail:expr, $msg:expr) => {
        if $ebail.is_err() { bail!($ebail, $msg) } else { $ebail }
    };
}

pub struct Bail {
    pub line: u32, // line number
    pub col: u32,  //
    pub msg: String,
    pub file: String,
    pub more: Option<Box<Bail>>,
}

impl Debug for Bail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{:?} ({:?}:{:?}): {:?} ", self.file, self.line, self.col, self.msg);
        if let Some(box bail) = &self.more {
            bail.fmt(f);
        } else {
            writeln!(f, "");
        }
        Ok(())
    }
}

impl std::convert::From<serde_json::error::Error> for Bail {
    fn from(e: serde_json::error::Error) -> Self {
        Bail { col: e.column() as u32,
               line: e.line() as u32,
               msg: format!("{:?}", e),
               file: "unknown".to_string(),
               more: None }
    }
}
