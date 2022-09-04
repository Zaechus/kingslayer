use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{entity::Entity, thing::Thing};

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc())
    }
}

#[derive(Deserialize, Serialize)]
pub enum Item {
    Thing(Thing),
}

impl Entity for Item {
    fn name(&self) -> &str {
        match self {
            Self::Thing(thing) => thing.name(),
        }
    }

    fn desc(&self) -> &str {
        match self {
            Self::Thing(thing) => thing.desc(),
        }
    }

    fn inspect(&self) -> &str {
        match self {
            Self::Thing(thing) => thing.inspect(),
        }
    }
}
