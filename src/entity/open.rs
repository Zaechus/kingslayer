use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(super) enum Opening {
    Open,
    Closed,
}

pub(crate) trait Closeable {
    fn open(&mut self) -> String;

    fn close(&mut self) -> String;

    fn is_closed(&self) -> bool;
}
