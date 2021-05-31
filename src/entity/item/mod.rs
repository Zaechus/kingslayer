use serde::{Deserialize, Serialize};

use crate::entity::Entity;

mod armor;
mod container;
mod gold;
mod key;
mod thing;
mod weapon;

pub use armor::Armor;
pub use container::Container;
pub use gold::Gold;
pub use key::Key;
pub use thing::Thing;
pub use weapon::Weapon;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Armor(Armor),
    Container(Container),
    Gold(Gold),
    Key(Key),
    Thing(Thing),
    Weapon(Weapon),
}

impl Item {
    pub fn long_name(&self) -> String {
        match self {
            Self::Armor(armor) => armor.name().to_owned(),
            Self::Container(container) => container.long_name(),
            Self::Gold(gold) => gold.name().to_owned(),
            Self::Key(key) => key.name().to_owned(),
            Self::Thing(thing) => thing.name().to_owned(),
            Self::Weapon(weapon) => weapon.name().to_owned(),
        }
    }

    pub fn long_desc(&self) -> String {
        match self {
            Self::Armor(armor) => armor.desc().to_owned(),
            Self::Container(container) => container.long_desc(),
            Self::Gold(gold) => gold.desc().to_owned(),
            Self::Key(key) => key.desc().to_owned(),
            Self::Thing(thing) => thing.desc().to_owned(),
            Self::Weapon(weapon) => weapon.desc().to_owned(),
        }
    }
}

impl Entity for Item {
    fn name(&self) -> &str {
        match self {
            Self::Armor(armor) => armor.name(),
            Self::Container(container) => container.name(),
            Self::Gold(gold) => gold.name(),
            Self::Key(key) => key.name(),
            Self::Thing(thing) => thing.name(),
            Self::Weapon(weapon) => weapon.name(),
        }
    }

    fn desc(&self) -> &str {
        match self {
            Self::Armor(armor) => armor.desc(),
            Self::Container(container) => container.desc(),
            Self::Gold(gold) => gold.desc(),
            Self::Key(key) => key.desc(),
            Self::Thing(thing) => thing.desc(),
            Self::Weapon(weapon) => weapon.desc(),
        }
    }

    fn inspect(&self) -> &str {
        match self {
            Self::Armor(armor) => armor.inspect(),
            Self::Container(container) => container.inspect(),
            Self::Gold(gold) => gold.inspect(),
            Self::Key(key) => key.inspect(),
            Self::Thing(thing) => thing.inspect(),
            Self::Weapon(weapon) => weapon.inspect(),
        }
    }
}
