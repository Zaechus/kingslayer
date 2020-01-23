use serde::{Deserialize, Serialize};

use super::{Closeable, Entity, Opening};
use crate::types::{Action, CmdResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pathway {
    directions: Vec<String>,
    target: String,
    #[serde(default)]
    desc: String,
    #[serde(default)]
    inspect: String,
    opening: Option<Opening>,
}

impl Pathway {
    pub fn long_desc(&self) -> String {
        if let Some(opening) = self.opening {
            match opening {
                Opening::Open => format!("{} The way is open.", self.desc),
                Opening::Closed => format!("{} The way is shut.", self.desc),
            }
        } else {
            self.desc.to_owned()
        }
    }

    pub fn directions(&self) -> &Vec<String> {
        &self.directions
    }
}

impl Entity for Pathway {
    fn name(&self) -> &str {
        &self.target
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn inspect(&self) -> &str {
        &self.inspect
    }
}

impl Closeable for Pathway {
    fn open(&mut self) -> CmdResult {
        if let Some(opening) = self.opening {
            if opening.is_closed() {
                self.opening = Some(Opening::Open);
                CmdResult::new(Action::Active, "Opened.".to_owned())
            } else {
                CmdResult::new(Action::Passive, String::from("The way is already open."))
            }
        } else {
            CmdResult::new(Action::Passive, String::from("The way is already open."))
        }
    }

    fn close(&mut self) -> CmdResult {
        if let Some(opening) = self.opening {
            if opening.is_open() {
                self.opening = Some(Opening::Closed);
                CmdResult::new(Action::Active, "Closed.".to_owned())
            } else {
                CmdResult::new(Action::Passive, String::from("The way cannot be closed."))
            }
        } else {
            CmdResult::new(Action::Passive, String::from("The way cannot be closed."))
        }
    }

    fn is_closed(&self) -> bool {
        if let Some(opening) = self.opening {
            opening.is_closed()
        } else {
            false
        }
    }
}
