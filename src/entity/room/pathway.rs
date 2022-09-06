use serde::{Deserialize, Serialize};

use crate::entity::open::{Closeable, Opening};

#[derive(Deserialize, Serialize)]
pub(crate) struct Pathway {
    pub(super) directions: Vec<String>,
    target: String,
    desc: String,
    inspect: String,
    #[serde(default)]
    door: Option<Opening>,
}

impl Pathway {
    pub(crate) fn target(&self) -> &str {
        &self.target
    }
}

impl Closeable for Pathway {
    fn open(&mut self) -> String {
        if let Some(Opening::Closed) = self.door {
            self.door = Some(Opening::Open);
            "Opened.".to_owned()
        } else {
            "It is already open!".to_owned()
        }
    }

    fn close(&mut self) -> String {
        match self.door {
            Some(Opening::Open) => {
                self.door = Some(Opening::Closed);
                "Closed.".to_owned()
            }
            Some(Opening::Closed) => "It is already closed!".to_owned(),
            None => "You cannot close that.".to_owned(),
        }
    }

    fn is_closed(&self) -> bool {
        matches!(self.door, Some(Opening::Closed))
    }
}
