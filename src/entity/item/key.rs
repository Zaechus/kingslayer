use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Key {
    name: String,
    desc: String,
    inspect: String,
    code: String,
}

impl Key {
    pub fn new(name: &str, inspect: &str, code: &str) -> Self {
        Self {
            name: name.to_owned(),
            desc: format!("There is a {} here.", name),
            inspect: inspect.to_owned(),
            code: code.to_owned(),
        }
    }
}

impl Entity for Key {
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
