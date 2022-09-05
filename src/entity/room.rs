use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::item::{item_index, Item};

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.name, self.desc)?;
        for i in self.items.iter() {
            write!(f, "\n{}", i)?;
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Room {
    name: String,
    desc: String,
    #[serde(default)]
    items: Vec<Item>,
}

impl Room {
    pub(crate) fn give<'a>(&mut self, item_name: &'a str) -> Result<Item, &'a str> {
        if let Some(pos) = item_index(&self.items, item_name) {
            Ok(self.items.remove(pos))
        } else {
            Err(item_name)
        }
    }

    pub(crate) fn take(&mut self, item: Result<Item, String>) -> String {
        match item {
            Ok(item) => {
                self.items.push(item);
                "Dropped.".to_owned()
            }
            Err(message) => message,
        }
    }
}
