use serde::{Deserialize, Serialize};

use crate::types::CmdResult;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Opening {
    Open,
    Closed,
}

impl Opening {
    pub fn is_open(self) -> bool {
        match self {
            Opening::Open => true,
            Opening::Closed => false,
        }
    }

    pub fn is_closed(self) -> bool {
        match self {
            Opening::Open => false,
            Opening::Closed => true,
        }
    }
}

pub trait Closeable {
    fn open(&mut self) -> CmdResult;

    fn close(&mut self) -> CmdResult;

    fn is_closed(&self) -> bool;
}
