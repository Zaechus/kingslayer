use serde::{Deserialize, Serialize};

use super::{Closeable, Entity};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pathway {
    target: String,
    #[serde(default)]
    desc: String,
    #[serde(default)]
    inspect: String,
    is_closed: Option<bool>,
    is_locked: Option<bool>,
}

impl Pathway {
    pub fn target(&self) -> &String {
        &self.target
    }

    pub fn desc(&self) -> String {
        if let Some(true) = self.is_closed {
            format!("{} The way is shut.", self.desc)
        } else if let Some(false) = self.is_closed {
            format!("{} The way is open.", self.desc)
        } else {
            self.desc.clone()
        }
    }

    pub fn is_closed(&self) -> Option<bool> {
        self.is_closed
    }

    pub fn is_locked(&self) -> Option<bool> {
        self.is_locked
    }
}

impl Entity for Pathway {
    fn name(&self) -> &String {
        &self.target
    }

    fn desc(&self) -> &String {
        &self.desc
    }

    fn inspect(&self) -> &String {
        &self.inspect
    }
}

impl Closeable for Pathway {
    fn open(&mut self) {
        if self.is_closed.is_some() {
            self.is_closed = Some(false)
        }
    }

    fn close(&mut self) {
        if self.is_closed.is_some() {
            self.is_closed = Some(true)
        }
    }
}
