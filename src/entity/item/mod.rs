use std::fmt::Display;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use self::thing::Thing;

use super::Entity;

mod thing;

/// Find the position of an Item in a collection of Items based on a name query
///
/// Given a room containging just an item with the name "red block":
/// > take red              =>  Taken.
/// > take block            =>  Taken.
/// > take red block        =>  Taken.
/// > take red red          =>  Taken.
/// > take block block      =>  Taken.
/// > take red block block  =>  Taken.
/// > take blue block       =>  There is no "blue block" here.
/// > take green plate      =>  There is no "green plate" here.
pub(crate) fn item_index(items: &[Item], item_name: &str) -> Option<usize> {
    items.par_iter().position_any(|item| {
        item_name
            .par_split_whitespace()
            .all(|word| item.name().contains(word))
    })
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc())
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) enum Item {
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
