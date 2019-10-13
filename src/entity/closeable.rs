use crate::types::CmdResult;

pub trait Closeable {
    fn open(&mut self) -> CmdResult;

    fn close(&mut self) -> CmdResult;

    fn is_closed(&self) -> bool;
}
