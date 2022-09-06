use std::fmt::Display;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use self::{container::Container, thing::Thing};

use super::Entity;

pub mod container;
mod thing;

/// Find the position of an Item in a collection of Items based on a name query
///
/// Given a room containing just an item with the name "red block":
/// > take red              =>  Taken.
/// > take block            =>  Taken.
/// > take red block        =>  Taken.
/// > take red red          =>  Taken.
/// > take block block      =>  Taken.
/// > take red block block  =>  Taken.
/// > take red plate        =>  There is no "red plate" here.
/// > take blue block       =>  There is no "blue block" here.
/// > take green plate      =>  There is no "green plate" here.
pub(crate) fn item_index(items: &[Item], item_name: &str) -> Option<usize> {
    items.par_iter().position_any(|item| {
        item_name.par_split_whitespace().all(|word| {
            item.name()
                .par_split_whitespace()
                .collect::<Vec<_>>()
                .contains(&word)
        })
    })
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Container(container) => write!(f, "{}", container),
            _ => write!(f, "{}", self.desc()),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) enum Item {
    Thing(Thing),
    Container(Container),
}

impl Entity for Item {
    fn name(&self) -> &str {
        match self {
            Self::Thing(thing) => thing.name(),
            Self::Container(container) => container.name(),
        }
    }

    fn desc(&self) -> &str {
        match self {
            Self::Thing(thing) => thing.desc(),
            Self::Container(container) => container.desc(),
        }
    }

    fn inspect(&self) -> &str {
        match self {
            Self::Thing(thing) => thing.inspect(),
            Self::Container(container) => container.inspect(),
        }
    }
}
