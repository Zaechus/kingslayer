use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::entity::{item::Item, Entity};

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.items.is_empty() {
            write!(f, "Your inventory is empty.")
        } else {
            write!(f, "You are carrying:")?;
            for i in self.items.iter() {
                write!(f, "\n  {}", i.name())?;
            }
            Ok(())
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Inventory {
    pub(super) items: Vec<Item>,
}
