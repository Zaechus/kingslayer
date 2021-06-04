use serde::{Deserialize, Serialize};

impl Default for Durability {
    fn default() -> Self {
        Durability::Unbreakable
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Durability {
    Durable(u32),
    Unbreakable,
}

impl Durability {}

pub trait Breakable {}
