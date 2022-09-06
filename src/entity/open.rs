use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize)]
pub(super) enum Opening {
    Open,
    Closed,
}

pub(super) trait Closeable {
    fn open(&mut self) -> String;

    fn close(&mut self) -> String;

    fn is_open(&self) -> bool;
}
