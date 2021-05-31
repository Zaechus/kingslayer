use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Armor {
    name: String,
    desc: String,
    inspect: String,
    ac: u32,
}

impl Armor {
    pub fn new(name: &str, inspect: &str, ac: u32) -> Self {
        Self {
            name: name.to_owned(),
            desc: format!("There is a {} here.", name),
            inspect: inspect.to_owned(),
            ac,
        }
    }

    pub const fn ac(&self) -> u32 {
        self.ac
    }
}

impl Entity for Armor {
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
