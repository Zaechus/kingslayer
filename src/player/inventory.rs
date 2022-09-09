use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::item::Item;

/// A collection of Items to be carried by a Player
#[derive(Default, Deserialize, Serialize)]
pub struct Inventory {
    pub(super) items: Vec<Item>,
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.items.is_empty() {
            write!(f, "Your inventory is empty.")
        } else {
            write!(
                f,
                "You are carrying:{}",
                self.items.iter().fold(String::new(), |acc, item| format!(
                    "{}\n  a {}",
                    acc,
                    item.name()
                ))
            )
        }
    }
}
