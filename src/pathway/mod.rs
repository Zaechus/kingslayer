use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::properties::IsOpen;

#[derive(Serialize, Deserialize)]
pub struct Pathway {
    name: String,
    desc: String,
    inspection: String,
    is_open: IsOpen,
}

impl Pathway {
    pub fn new(name: &str, desc: &str, inspection: &str, is_open: IsOpen) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            inspection: inspection.to_owned(),
            is_open,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn desc(&self) -> String {
        self.desc.clone()
    }
    pub fn inspection(&self) -> String {
        self.inspection.clone()
    }
}
