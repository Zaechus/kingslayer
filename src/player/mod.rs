use serde::{Deserialize, Serialize};

use crate::entity::item::{item_index, Item};

use self::inventory::Inventory;

mod inventory;

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Player {
    inventory: Inventory,
}

impl Player {
    pub(crate) fn drop(&mut self, item_name: &str) -> Result<Item, String> {
        match item_index(&self.inventory.items, item_name) {
            Some(pos) => Ok(self.inventory.items.remove(pos)),
            None => Err(format!("You do not have the \"{}\".", item_name)),
        }
    }

    pub(crate) fn take(&mut self, item: Result<Item, &str>) -> String {
        match item {
            Ok(item) => {
                self.inventory.items.push(item);
                "Taken.".to_owned()
            }
            Err(item_name) => format!("There is no \"{}\" here.", item_name),
        }
    }

    pub(crate) const fn inventory(&self) -> &Inventory {
        &self.inventory
    }
}
