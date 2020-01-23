use serde::{Deserialize, Serialize};

use super::Entity;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ally {
    name: String,
    desc: String,
    inspect: String,
    hp: i32,
}

impl Ally {}

impl Entity for Ally {
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
