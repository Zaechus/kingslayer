use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pathway {
    name: String,
    desc: String,
    inspection: String,
    is_closed: Option<bool>,
    is_locked: Option<bool>,
}

impl Pathway {
    pub fn name(&self) -> &String {
        &self.name
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

    pub fn inspection(&self) -> &String {
        &self.inspection
    }

    pub fn open(&mut self) {
        if self.is_closed.is_some() {
            self.is_closed = Some(false)
        }
    }

    pub fn close(&mut self) {
        if self.is_closed.is_some() {
            self.is_closed = Some(true)
        }
    }

    pub fn is_closed(&self) -> Option<bool> {
        self.is_closed
    }

    pub fn is_locked(&self) -> Option<bool> {
        self.is_locked
    }
}
