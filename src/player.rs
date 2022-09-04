use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{entity::Entity, item::Item, item_index};

#[derive(Default, Deserialize, Serialize)]
pub struct Player {
    inventory: Inventory,
}

impl Player {
    pub fn drop(&mut self, item_name: &str) -> Result<Item, String> {
        if let Some(pos) = item_index(&self.inventory.items, item_name) {
            Ok(self.inventory.items.remove(pos))
        } else {
            Err(format!("You do not have the \"{}\".", item_name))
        }
    }

    pub fn take(&mut self, item: Result<Item, &str>) -> String {
        match item {
            Ok(item) => {
                self.inventory.items.push(item);
                "Taken.".to_owned()
            }
            Err(item_name) => format!("There is no \"{}\" here.", item_name),
        }
    }

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
                write!(f, "  {}", i.name())?;
            }
            Ok(())
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Inventory {
    items: Vec<Item>,
}
