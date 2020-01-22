use serde::{Deserialize, Serialize};

use crate::entity::Entity;

// An object to be interacted with by the user
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Element {
    name: String,
    desc: String,
    inspect: String,
}

impl Element {
    pub fn new(name: &str, desc: &str, inspect: &str) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            inspect: inspect.to_owned(),
        }
    }

    pub fn with_desc(mut self, desc: &str) -> Self {
        self.desc = String::from(desc);
        self
    }
}

impl Entity for Element {
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
