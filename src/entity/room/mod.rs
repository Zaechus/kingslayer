use std::fmt::Display;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use self::pathway::Pathway;

use super::item::{item_index, Item};

mod pathway;

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
    paths: Vec<Pathway>,
    #[serde(default)]
    items: Vec<Item>,
}

impl Room {
    /// Find a Pathway in the Room by its directions given a direction query.
    /// All query words must match but not all directions must be present in the query.
    /// See the docs for item_index
    pub(crate) fn find_path(&self, direction: &str) -> Option<&Pathway> {
        self.paths.par_iter().find_any(|path| {
            direction
                .par_split_whitespace()
                .all(|word| path.directions.contains(&word.to_owned()))
        })
    }

    pub(crate) fn give<'a>(&mut self, item_name: &'a str) -> Result<Item, &'a str> {
        match item_index(&self.items, item_name) {
            Some(pos) => Ok(self.items.remove(pos)),
            None => Err(item_name),
        }
    }

    pub(crate) fn give_all(&mut self) -> Vec<Item> {
        self.items.par_drain(..).collect()
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
