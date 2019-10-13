use serde::{Deserialize, Serialize};

use super::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ally {
    hp: i32,
    name: String,
    desc: String,
    inspect: String,
}

impl Ally {}

impl Entity for Ally {
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
