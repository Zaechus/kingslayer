use serde::{Deserialize, Serialize};

use crate::entity::Entity;

// An object to be interacted with by the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Gold {
    name: String,
    desc: String,
    inspect: String,
    amount: u32,
}

impl Gold {
    pub fn amount(&self) -> u32 {
        self.amount
    }
}

impl Entity for Gold {
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
