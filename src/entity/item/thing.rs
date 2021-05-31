use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Thing {
    name: String,
    desc: String,
    inspect: String,
}

impl Thing {
    pub fn new(name: &str, inspect: &str) -> Self {
        Self {
            name: name.to_owned(),
            desc: format!("There is a {} here.", name),
            inspect: inspect.to_owned(),
        }
    }

    pub fn with_desc(mut self, desc: &str) -> Self {
        self.desc = String::from(desc);
        self
    }
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
