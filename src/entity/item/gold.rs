use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gold {
    name: String,
    desc: String,
    inspect: String,
    amount: u32,
}

impl Gold {
    pub fn new(amount: u32) -> Self {
        Self {
            name: String::from("gold"),
            desc: format!("There is {} gold here.", amount),
            inspect: String::from("It is shiny and smooth"),
            amount,
        }
    }

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
