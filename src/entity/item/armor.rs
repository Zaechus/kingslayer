use serde::{Deserialize, Serialize};

use crate::entity::Entity;

// An object to be interacted with by the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Armor {
    name: String,
    desc: String,
    inspect: String,
    ac: u32,
}

impl Armor {
    pub fn ac(&self) -> u32 {
        self.ac
    }
}

impl Entity for Armor {
    fn name(&self) -> &String {
        &self.name
    }

    fn desc(&self) -> &String {
        &self.desc
    }

    fn inspect(&self) -> &String {
        &self.inspect
    }
}
