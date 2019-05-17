use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ally {
    hp: i32,
    name: String,
    desc: String,
    inspection: String,
}

impl Ally {
    pub fn desc(&self) -> &String {
        &self.desc
    }

    pub fn inspection(&self) -> &String {
        &self.inspection
    }
}
