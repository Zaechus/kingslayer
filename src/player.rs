use rand::Rng;

use serde_derive::{Deserialize, Serialize};

use crate::{
    entities::Item,
    types::{CmdResult, ItemMap},
};

#[derive(Serialize, Deserialize)]
pub struct Player {
    hp: (i32, i32),
    xp: (i32, i32),
    in_combat: bool,
    lvl: i32,
    stat_pts: i32,
    strength: i32,
    dex: i32,
    con: i32,
    int: i32,
    wis: i32,
    cha: i32,
    main_hand: Option<Box<Item>>,
    inventory: ItemMap,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hp: (13, 13),
            xp: (0, 1000),
            in_combat: false,
            lvl: 1,
            stat_pts: 4,
            strength: 13,
            dex: 13,
            con: 13,
            int: 13,
            wis: 13,
            cha: 13,
            main_hand: None,
            inventory: ItemMap::new(),
        }
    }

    pub fn wait() -> CmdResult {
        CmdResult::new(true, "Time passes...".to_string())
    }

    pub fn hp(&self) -> i32 {
        self.hp.0
    }

    pub fn hp_cap(&self) -> i32 {
        self.hp.1
    }

    pub fn is_alive(&self) -> bool {
        self.hp.0 > 0
    }

    pub fn level_up(&mut self) -> String {
        if self.xp.0 >= self.xp.1 {
            self.xp.0 -= self.xp.1;
            self.xp.1 = 1800 * (self.lvl - 2).pow(2) + 1000;
            self.lvl += 1;
            self.stat_pts += self.lvl + 3;
            format!("You advanced to level {}!", self.lvl)
        } else {
            String::new()
        }
    }

    pub fn gain_xp(&mut self, gained: i32) {
        self.xp.0 += gained;
    }

    pub fn increase_ability_score(&mut self, ability_score: &str) -> CmdResult {
        if self.stat_pts >= 0 {
            match &ability_score[0..3] {
                "str" | "dex" | "con" | "int" | "wis" | "cha" => {
                    self.stat_pts -= 1;
                    CmdResult::new(true, "Ability increased by one.".to_string())
                }
                _ => CmdResult::new(
                    false,
                    format!("\"{}\" is not a valid ability score.", ability_score),
                ),
            }
        } else {
            CmdResult::new(false, "You do not have any stat points.".to_string())
        }
    }

    pub fn engage_combat(&mut self) {
        self.in_combat = true;
    }

    pub fn disengage_combat(&mut self) {
        self.in_combat = false;
    }

    pub fn attack(&mut self) -> Option<i32> {
        if let Some(weapon) = &self.main_hand {
            self.in_combat = true;
            Some(self.deal_damage(weapon.damage()))
        } else {
            None
        }
    }

    pub fn attack_with(&mut self, weapon_name: &str) -> Option<i32> {
        if let Some(weapon) = self.inventory.get(weapon_name) {
            self.in_combat = true;
            Some(self.deal_damage(weapon.damage()))
        } else if let Some(equipped_weapon) = &self.main_hand {
            if weapon_name == equipped_weapon.name() {
                self.in_combat = true;
                Some(self.deal_damage(equipped_weapon.damage()))
            } else {
                None
            }
        } else {
            None
        }
    }

    // rest for a random amount of time to regain a random amount of HP
    pub fn rest(&mut self) -> CmdResult {
        if self.hp() < self.hp_cap() {
            if !self.in_combat {
                let regained_hp = rand::thread_rng().gen_range(1, 7);
                let new_hp = self.hp() + regained_hp;
                if new_hp < self.hp_cap() {
                    self.hp = (new_hp, self.hp_cap());
                } else {
                    self.hp = (self.hp_cap(), self.hp_cap());
                }
                CmdResult::new(
                    true,
                    format!(
                        "You regained {} HP for a total of ({} / {}) HP.",
                        regained_hp,
                        self.hp(),
                        self.hp_cap()
                    ),
                )
            } else {
                CmdResult::new(false, "You cannot rest while in combat.".to_string())
            }
        } else {
            CmdResult::new(false, "You already have full health.".to_string())
        }
    }

    pub fn inventory(&self) -> CmdResult {
        if self.inventory.is_empty() && self.main_hand.is_none() {
            CmdResult::new(true, "You are empty-handed.".to_string())
        } else {
            let mut items_carried = String::new();
            if let Some(weapon) = &self.main_hand {
                items_carried.push_str(&format!("Main hand: {}\n", weapon.name()));
            }
            if !self.inventory.is_empty() {
                items_carried.push_str("You are carrying:");
                for x in self.inventory.iter() {
                    items_carried = format!("{}\n  {}", items_carried, x.1.name());
                }
            }
            CmdResult::new(true, items_carried)
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.inventory.contains_key(name)
    }

    pub fn main_hand(&self) -> &Option<Box<Item>> {
        &self.main_hand
    }

    pub fn status(&self) -> CmdResult {
        CmdResult::new(
            false,
            format!(
                "Level: {}\nHP: ({} / {})\nXP: ({} / {})\nStat points: {}",
                self.lvl,
                self.hp(),
                self.hp_cap(),
                self.xp.0,
                self.xp.1,
                self.stat_pts
            ),
        )
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp = (self.hp.0 - damage, self.hp.1);
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if name == "me" || name == "self" || name == "myself" {
            Some(self.status())
        } else if let Some(item) = self.inventory.get(name) {
            Some(CmdResult::new(true, item.inspection().to_string()))
        } else if let Some(item) = &self.main_hand {
            Some(CmdResult::new(true, item.inspection().to_string()))
        } else {
            None
        }
    }

    pub fn take(&mut self, name: &str, item: Option<Box<Item>>) -> CmdResult {
        if let Some(obj) = item {
            let mut res = String::from("Taken.");
            if obj.is_weapon() {
                res.push_str("\n(You can equip weapons with \"equip\" or \"draw\")");
            }
            self.inventory.insert(name.to_string(), obj);
            CmdResult::new(true, res)
        } else {
            CmdResult::new(
                false,
                format!(
                    "There is no \"{}\" here. Make sure you are being specific.",
                    name
                ),
            )
        }
    }

    // take an Item from a container Item in the inventory
    pub fn take_from(&mut self, item: &str, container: &str) -> CmdResult {
        if let Some(cont) = self.inventory.get_mut(container) {
            if let Some(ref mut contents) = cont.contents_mut() {
                if let Some(itm) = contents.remove(item) {
                    self.inventory.insert(item.to_string(), itm);
                    CmdResult::new(true, "Taken.".to_string())
                } else {
                    CmdResult::new(
                        true,
                        format!("There is no \"{}\" inside of the \"{}\".", item, container),
                    )
                }
            } else {
                CmdResult::new(
                    false,
                    format!("You cannot take anything from the \"{}\".", container),
                )
            }
        } else {
            CmdResult::new(false, format!("You do not have the \"{}\".", container))
        }
    }

    pub fn take_all(&mut self, items: ItemMap) -> CmdResult {
        if items.is_empty() {
            CmdResult::new(false, "There is nothing to take.".to_string())
        } else {
            let times = items.len();
            self.inventory.extend(items);

            let mut res = String::new();
            for _ in 0..times {
                res.push_str("Taken. ");
            }
            CmdResult::new(true, res)
        }
    }

    // remove an item from inventory and into the current Room
    pub fn remove(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.inventory.remove(name) {
            Some(item)
        } else if let Some(item) = self.main_hand.take() {
            Some(item)
        } else {
            None
        }
    }

    // equip an Item into main_hand to simplify fighting
    pub fn equip(&mut self, weapon_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.remove(weapon_name) {
            // move old main hand back to inventory
            if let Some(weapon) = self.main_hand.take() {
                self.take(weapon_name, Some(weapon));
            }
            self.main_hand = Some(item);
            CmdResult::new(
                true,
                "Equipped.\n(You can unequip items with \"drop\" or by equipping a different item)"
                    .to_string(),
            )
        } else {
            CmdResult::new(false, format!("You do not have the \"{}\".", weapon_name))
        }
    }

    // place an item into a container item
    pub fn put_in(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.remove(item_name) {
            if let Some(container) = self.inventory.get_mut(container_name) {
                if let Some(ref mut contents) = container.contents_mut() {
                    contents.insert(item_name.to_string(), item);
                    CmdResult::new(true, "Placed.".to_string())
                } else {
                    CmdResult::new(
                        false,
                        format!("You cannot put anything in the {}.", container_name),
                    )
                }
            } else {
                CmdResult::new(
                    false,
                    format!("You do not have the \"{}\".", container_name),
                )
            }
        } else {
            CmdResult::new(false, format!("You do not have the \"{}\".", item_name))
        }
    }

    fn deal_damage(&self, weapon_damage: i32) -> i32 {
        weapon_damage + ((self.strength - 10) as f32 / 2.0).floor() as i32
    }
}