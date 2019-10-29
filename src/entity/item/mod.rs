use serde::Deserialize;

use crate::entity::Entity;

mod armor;
mod container;
mod thing;
mod weapon;

pub use armor::Armor;
pub use container::Container;
pub use thing::Thing;
pub use weapon::Weapon;

#[derive(Debug, Deserialize)]
pub enum Item {
    Armor(Armor),
    Container(Container),
    Thing(Thing),
    Weapon(Weapon),
}

impl Item {
    pub fn long_name(&self) -> String {
        match self {
            Item::Armor(armor) => armor.name().to_owned(),
            Item::Container(container) => container.long_name(),
            Item::Thing(thing) => thing.name().to_owned(),
            Item::Weapon(weapon) => weapon.name().to_owned(),
        }
    }

    pub fn long_desc(&self) -> String {
        match self {
            Item::Armor(armor) => armor.desc().to_owned(),
            Item::Container(container) => container.long_desc(),
            Item::Thing(thing) => thing.desc().to_owned(),
            Item::Weapon(weapon) => weapon.desc().to_owned(),
        }
    }
}

impl Entity for Item {
    fn name(&self) -> &str {
        match self {
            Item::Armor(armor) => armor.name(),
            Item::Container(container) => container.name(),
            Item::Thing(thing) => thing.name(),
            Item::Weapon(weapon) => weapon.name(),
        }
    }

    fn desc(&self) -> &str {
        match self {
            Item::Armor(armor) => armor.desc(),
            Item::Container(container) => container.desc(),
            Item::Thing(thing) => thing.desc(),
            Item::Weapon(weapon) => weapon.desc(),
        }
    }

    fn inspect(&self) -> &str {
        match self {
            Item::Armor(armor) => armor.inspect(),
            Item::Container(container) => container.inspect(),
            Item::Thing(thing) => thing.inspect(),
            Item::Weapon(weapon) => weapon.inspect(),
        }
    }
}
