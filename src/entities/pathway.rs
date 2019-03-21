use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Pathway {
    name: String,
    desc: String,
    inspection: String,
    pub is_open: bool,
    is_locked: bool,
}

impl Pathway {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn desc(&self) -> String {
        self.desc.clone()
    }

    pub fn inspection(&self) -> String {
        self.inspection.clone()
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }
}
