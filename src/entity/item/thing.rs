use serde::{Deserialize, Serialize};

use crate::entity::Entity;

// An object to be interacted with by the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Thing {
    name: String,
    desc: String,
    inspect: String,
}

impl Entity for Thing {
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
