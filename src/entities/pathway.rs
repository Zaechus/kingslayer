use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pathway {
    name: String,
    desc: String,
    inspection: String,
    is_open: bool,
    is_locked: bool,
}

impl Pathway {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn desc(&self) -> String {
        if self.is_open {
            format!("{} The way is open.", self.desc)
        } else {
            format!("{} The way is shut.", self.desc)
        }
    }

    pub fn inspection(&self) -> &String {
        &self.inspection
    }

    pub fn open(&mut self) {
        self.is_open = true
    }

    pub fn close(&mut self) {
        self.is_open = false
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }
}
