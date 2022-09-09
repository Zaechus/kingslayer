use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{exit::Exit, item::Item};

/// An explorable location
#[derive(Deserialize, Serialize)]
pub struct Room {
    name: String,
    desc: String,
    exits: Vec<Exit>,
    items: Vec<Item>,
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.name, self.desc)
    }
}

impl Room {
    pub fn new<S: Into<String>>(name: S, desc: S) -> Self {
        Self {
            name: name.into(),
            desc: desc.into(),
            exits: Vec::new(),
            items: Vec::new(),
        }
    }
}
