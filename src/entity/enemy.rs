use serde::{Deserialize, Serialize};

use super::{Entity, Item};
use crate::{
    dice_roll,
    types::{Action, CmdResult, EnemyStatus, Items},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enemy {
    name: String,
    desc: String,
    inspect: String,
    hp: i32,
    ac: i32,
    xp: u32,
    damage: u32,
    status: EnemyStatus,
    #[serde(default)]
    loot: Items,
}

impl Enemy {
    pub fn new(name: &str, inspect: &str, status: EnemyStatus) -> Self {
        Self {
            name: name.to_owned(),
            desc: format!("There is a {} here.", name),
            inspect: inspect.to_owned(),
            hp: 1,
            ac: 0,
            xp: 0,
            damage: 1,
            status,
            loot: Items::new(),
        }
    }
    pub fn new_rats(status: EnemyStatus) -> Self {
        Self {
            name: String::from("swarm of rats"),
            desc: String::from("A swarm of rats raves along the floor."),
            inspect: String::from("The creatures chatter and scrape viciously."),
            hp: 24,
            ac: 0,
            xp: 25,
            damage: 2,
            status,
            loot: Items::new(),
        }
    }

    pub fn new_pirate(status: EnemyStatus) -> Self {
        Self {
            name: String::from("pirate"),
            desc: String::from("There is a pirate here."),
            inspect: String::from("The pirate is armed and smells vile."),
            hp: dice_roll(2, 7) as i32 + 2,
            ac: 10,
            xp: 50,
            damage: 6,
            status,
            loot: Items::new(),
        }
    }

    pub fn with_desc(mut self, desc: &str) -> Self {
        self.desc = String::from(desc);
        self
    }
    pub fn with_inspect(mut self, inspect: &str) -> Self {
        self.inspect = String::from(inspect);
        self
    }
    pub fn with_hp(mut self, hp: i32) -> Self {
        self.hp = hp;
        self
    }
    pub fn with_ac(mut self, ac: i32) -> Self {
        self.ac = ac;
        self
    }
    pub fn with_xp(mut self, xp: u32) -> Self {
        self.xp = xp;
        self
    }
    pub fn with_damage(mut self, damage: u32) -> Self {
        self.damage = damage;
        self
    }
    pub fn with_item(mut self, item: Item) -> Self {
        self.loot.push(Box::new(item));
        self
    }

    pub fn long_desc(&self) -> String {
        match self.status {
            EnemyStatus::Asleep => format!("{} It is asleep.", self.desc),
            _ => self.desc.to_owned(),
        }
    }

    pub const fn xp(&self) -> u32 {
        self.xp
    }

    pub fn damage(&self) -> u32 {
        dice_roll(1, self.damage)
    }

    pub fn is_angry(&self) -> bool {
        self.status == EnemyStatus::Angry
    }

    pub const fn status(&self) -> EnemyStatus {
        self.status
    }

    pub fn make_angry(&mut self) {
        self.status = EnemyStatus::Angry;
    }

    pub const fn loot(&self) -> &Items {
        &self.loot
    }

    pub fn take_damage(&mut self, damage: u32) -> Option<CmdResult> {
        self.make_angry();

        if dice_roll(1, 20) as i32 >= self.ac {
            self.hp -= damage as i32;
            None
        } else {
            match dice_roll(1, 3) {
                0 => Some(CmdResult::new(
                    Action::Active,
                    format!(
                        "\nYou swung at the {}, but it dodged out of the way.",
                        self.name
                    ),
                )),
                1 => Some(CmdResult::new(
                    Action::Active,
                    format!(
                        "\nYou hit the {}, but its armor absorbed the blow.",
                        self.name
                    ),
                )),
                _ => Some(CmdResult::new(
                    Action::Active,
                    format!("\nThe {} deftly blocked your attack.", self.name),
                )),
            }
        }
    }

    pub const fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn drop_loot(&mut self) -> Items {
        self.loot.drain(0..).collect()
    }
}

impl Entity for Enemy {
    fn name(&self) -> &str {
        &self.name
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn inspect(&self) -> &str {
        &self.inspect
    }
}
