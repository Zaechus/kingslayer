use std::fmt::Display;

use serde::{Deserialize, Serialize};

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.name, self.desc)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Room {
    name: String,
    desc: String,
    #[serde(default)]
    items: Vec<String>,
}
