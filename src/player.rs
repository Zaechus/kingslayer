use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Player {
    inventory: Inventory,
}

impl Player {
    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.items.is_empty() {
            write!(f, "Your inventory is empty.")
        } else {
            writeln!(f, "You are carrying:")?;
            for i in self.items.iter() {
                write!(f, "  {}", i)?;
            }
            Ok(())
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Inventory {
    items: Vec<String>,
}
