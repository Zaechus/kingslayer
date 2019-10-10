use serde_derive::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ally {
    hp: i32,
    name: String,
    desc: String,
    inspection: String,
}

impl Ally {}

impl Entity for Ally {
    fn name(&self) -> &String {
        &self.name
    }

    fn desc(&self) -> &String {
        &self.desc
    }

    fn inspection(&self) -> &String {
        &self.inspection
    }
}
