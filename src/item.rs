use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Item {
    name: String,
    desc: String,
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }
}
