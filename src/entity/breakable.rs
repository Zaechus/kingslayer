use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Durability {
    Durable(u32),
    #[default]
    Unbreakable,
}

impl Durability {}

// pub trait Breakable {}
