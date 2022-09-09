use serde::{Deserialize, Serialize};

use self::inventory::Inventory;

mod inventory;

/// Represents a player's current position and Inventory
#[derive(Deserialize, Serialize)]
pub struct Player {
    location: String,
    inventory: Inventory,
}

impl Player {
    pub fn new<S: Into<String>>(location: S) -> Self {
        Self {
            location: location.into(),
            inventory: Inventory::default(),
        }
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }
}
