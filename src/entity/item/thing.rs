use serde::Deserialize;

use crate::entity::Entity;

// An object to be interacted with by the user
#[derive(Debug, Deserialize)]
pub struct Thing {
    name: String,
    desc: String,
    inspect: String,
}

impl Entity for Thing {
    fn name(&self) -> &str {
        &self.name
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn inspect(&self) -> &str {
        &self.inspect
    }
}
