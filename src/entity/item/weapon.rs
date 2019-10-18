use rand::Rng;

use serde::{Deserialize, Serialize};

use crate::entity::Entity;

// An object to be interacted with by the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    name: String,
    desc: String,
    inspect: String,
    damage: u32,
}

impl Weapon {
    pub fn damage(&self) -> u32 {
        rand::thread_rng().gen_range(1, self.damage + 1)
    }
}

impl Entity for Weapon {
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
