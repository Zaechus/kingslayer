use rand::Rng;

use serde_derive::{Deserialize, Serialize};

use crate::{
    entity::Item,
    response::dont_have,
    stats::Stats,
    types::{CmdResult, ItemMap},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    hp: (i32, u32),
    xp: (u32, u32),
    in_combat: bool,
    lvl: u32,
    stats: Stats,
    main_hand: Option<Box<Item>>,
    armor: Option<Box<Item>>,
    inventory: ItemMap,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hp: (13, 13),
            xp: (0, 1000),
            in_combat: false,
            lvl: 1,
            stats: Stats::new(),
            main_hand: None,
            armor: None,
            inventory: ItemMap::new(),
        }
    }

    fn ac(&self) -> u32 {
        if let Some(armor) = &self.armor {
            if let Some(ac) = armor.armor_class() {
                ac + self.stats.dex
            } else {
                10 + self.stats.dex
            }
        } else {
            10 + self.stats.dex
        }
    }

    pub fn attack(&mut self) -> Option<u32> {
        if let Some(weapon) = &self.main_hand {
            self.in_combat = true;
            Some(self.deal_damage(weapon.damage()))
        } else {
            None
        }
    }

    pub fn attack_with(&mut self, weapon_name: &str) -> Option<u32> {
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

    pub fn close(&mut self, item_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.get_mut(item_name) {
            if item.is_closed() == Some(true) {
                CmdResult::new(false, format!("The {} is already closed.", item_name))
            } else {
                item.close();
                CmdResult::new(true, "Closed.".to_owned())
            }
        } else {
            CmdResult::new(false, format!("There is no \"{}\".", item_name))
        }
    }

    fn deal_damage(&self, weapon_damage: u32) -> u32 {
        weapon_damage + self.stats.strngth
    }

    pub fn disengage_combat(&mut self) {
        self.in_combat = false;
    }

    pub fn don_armor(&mut self, armor_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.remove(armor_name) {
            if item.armor_class().is_some() {
                // move old armor back to inventory
                if let Some(armor) = self.armor.take() {
                    self.take(armor_name, Some(armor));
                }
                self.armor = Some(item);
                CmdResult::new(
                true,
                "Donned.\n(You can remove armor with \"drop\" or by donning a different set or armor)"
                    .to_owned(),
            )
            } else {
                self.inventory.insert(armor_name.to_owned(), item);
                CmdResult::new(
                    false,
                    format!(
                        "You cannot put on the \"{}\", which isn't armor.",
                        armor_name
                    ),
                )
            }
        } else {
            dont_have(armor_name)
        }
    }

    pub fn engage_combat(&mut self) {
        self.in_combat = true;
    }

    // equip an Item into main_hand to simplify fighting
    pub fn equip(&mut self, weapon_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.remove(weapon_name) {
            // move old main hand back to inventory
            if let Some(weapon) = self.main_hand.take() {
                self.take(&weapon.name(), Some(weapon));
            }
            if item.armor_class().is_none() {
                self.main_hand = Some(item);
                CmdResult::new(
                    true,
                    "Equipped.\n(You can unequip items with \"drop\" or by equipping a different item)"
                        .to_owned(),
                )
            } else {
                self.take(weapon_name, Some(item));
                CmdResult::new(
                    false,
                    "You cannot equip armor. Use \"don\" instead.".to_owned(),
                )
            }
        } else {
            dont_have(weapon_name)
        }
    }

    pub fn gain_xp(&mut self, gained: u32) {
        self.xp.0 += gained;
    }

    pub fn has(&self, name: &str) -> bool {
        self.inventory.contains_key(name)
    }

    pub fn hp(&self) -> i32 {
        self.hp.0
    }

    pub fn hp_cap(&self) -> u32 {
        self.hp.1
    }

    pub fn increase_ability_mod(&mut self, ability_score: &str) -> CmdResult {
        if self.stats.pts > 0 {
            match &ability_score[0..3] {
                "str" | "dex" | "con" | "int" | "wis" | "cha" => {
                    self.stats.pts -= 1;
                    CmdResult::new(true, "Ability modifier increased by one.".to_owned())
                }
                _ => CmdResult::new(
                    false,
                    format!("\"{}\" is not a valid ability score.", ability_score),
                ),
            }
        } else {
            CmdResult::new(false, "You do not have any stat points.".to_owned())
        }
    }

    // place an item into a container item
    pub fn insert_into(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        let is_closed = if let Some(container) = self.inventory.get(container_name) {
            container.is_closed()
        } else {
            return dont_have(container_name);
        };
        if is_closed == Some(true) {
            CmdResult::new(false, format!("The {} is closed.", container_name))
        } else if let Some(item) = self.inventory.remove(item_name) {
            if let Some(container) = self.inventory.get_mut(container_name) {
                if let Some(ref mut contents) = container.contents_mut() {
                    contents.insert(item_name.to_owned(), item);
                    CmdResult::new(true, "Placed.".to_owned())
                } else {
                    CmdResult::new(
                        false,
                        format!("You cannot put anything in the {}.", container_name),
                    )
                }
            } else {
                dont_have(container_name)
            }
        } else {
            CmdResult::new(false, format!("You do not have the \"{}\".", item_name))
        }
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if name == "me" || name == "self" || name == "myself" {
            Some(self.status())
        } else if let Some(item) = self.inventory.get(name) {
            Some(CmdResult::new(true, item.inspection().to_owned()))
        } else if let Some(item) = &self.main_hand {
            Some(CmdResult::new(true, item.inspection().to_owned()))
        } else {
            None
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp.0 > 0
    }

    pub fn level_up(&mut self) -> String {
        if self.xp.0 >= self.xp.1 {
            self.xp.0 -= self.xp.1;
            self.xp.1 = 1800 * (self.lvl - 2).pow(2) + 1000;
            self.lvl += 1;
            self.stats.pts += self.lvl + 3;
            format!("You advanced to level {}!", self.lvl)
        } else {
            String::new()
        }
    }

    pub fn main_hand(&self) -> &Option<Box<Item>> {
        &self.main_hand
    }

    pub fn open(&mut self, item_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.get_mut(item_name) {
            if item.is_closed() == Some(true) {
                item.open();
                CmdResult::new(true, "Opened.".to_owned())
            } else {
                CmdResult::new(false, format!("The {} is already opened.", item_name))
            }
        } else {
            CmdResult::new(false, format!("There is no \"{}\".", item_name))
        }
    }

    pub fn print_inventory(&self) -> CmdResult {
        let mut items_carried = String::new();
        if let Some(weapon) = &self.main_hand {
            items_carried.push_str(&format!("Main hand: {}\n", weapon.name()));
        }
        if let Some(armor) = &self.armor {
            items_carried.push_str(&format!("Armor: {}\n", armor.name()));
        }
        if self.inventory.is_empty() {
            items_carried.push_str("Your inventory is empty.");
        } else {
            items_carried.push_str("You are carrying:");
            for x in self.inventory.iter() {
                items_carried = format!("{}\n  {}", items_carried, x.1.name());
            }
        }
        CmdResult::new(true, items_carried)
    }

    // remove an item from inventory and into the current Room
    pub fn remove(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.inventory.remove(name) {
            Some(item)
        } else if let Some(item) = self.main_hand.take() {
            Some(item)
        } else if let Some(item) = self.armor.take() {
            Some(item)
        } else {
            None
        }
    }

    // rest for a random amount of time to regain a random amount of HP
    pub fn rest(&mut self) -> CmdResult {
        if self.hp() < self.hp_cap() as i32 {
            if !self.in_combat {
                let regained_hp = rand::thread_rng().gen_range(1, 7);
                let new_hp = self.hp() + regained_hp;
                if new_hp < self.hp_cap() as i32 {
                    self.hp = (new_hp, self.hp_cap());
                } else {
                    self.hp = (self.hp_cap() as i32, self.hp_cap());
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
                CmdResult::new(false, "You cannot rest while in combat.".to_owned())
            }
        } else {
            CmdResult::new(false, "You already have full health.".to_owned())
        }
    }

    pub fn stats(&self) -> CmdResult {
        CmdResult::new(true, self.stats.print_stats())
    }

    pub fn status(&self) -> CmdResult {
        CmdResult::new(
            false,
            format!(
                "Level: {}\nHP: ({} / {})\nAC: {}\nXP: ({} / {})\nStat points: {}",
                self.lvl,
                self.hp(),
                self.hp_cap(),
                self.ac(),
                self.xp.0,
                self.xp.1,
                self.stats.pts
            ),
        )
    }

    pub fn take_damage(&mut self, enemy_name: &str, damage: u32) -> String {
        if rand::thread_rng().gen_range(1, 21) > self.ac() {
            self.hp = (self.hp.0 - damage as i32, self.hp.1);
            format!(
                "\nThe {} hit you for {} damage. You have {} HP left.",
                enemy_name,
                damage,
                self.hp()
            )
        } else {
            match rand::thread_rng().gen_range(0, 3) {
                0 => format!(
                    "\nThe {} swung at you, but you dodged out of the way.",
                    enemy_name
                ),
                1 => format!(
                    "\nThe {} hit you, but your armor deflected the blow.",
                    enemy_name
                ),
                _ => format!(
                    "\nThe {} struck at you, but you deftly blocked the blow.",
                    enemy_name
                ),
            }
        }
    }

    pub fn take(&mut self, name: &str, item: Option<Box<Item>>) -> CmdResult {
        if let Some(obj) = item {
            let mut res = String::from("Taken.");
            if obj.is_weapon() {
                res.push_str("\n(You can equip weapons with \"equip\" or \"draw\")");
            }
            self.inventory.insert(name.to_owned(), obj);
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

    pub fn take_all(&mut self, items: ItemMap) -> CmdResult {
        if items.is_empty() {
            CmdResult::new(false, "There is nothing to take.".to_owned())
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

    // take an Item from a container Item in the inventory
    pub fn take_from(&mut self, item: &str, container_name: &str) -> CmdResult {
        let is_closed = if let Some(container) = self.inventory.get(container_name) {
            container.is_closed()
        } else {
            return dont_have(container_name);
        };
        if is_closed == Some(true) {
            CmdResult::new(false, format!("The {} is closed.", container_name))
        } else if let Some(container) = self.inventory.get_mut(container_name) {
            if let Some(ref mut contents) = container.contents_mut() {
                if let Some(itm) = contents.remove(item) {
                    self.inventory.insert(item.to_owned(), itm);
                    CmdResult::new(true, "Taken.".to_owned())
                } else {
                    CmdResult::new(
                        true,
                        format!(
                            "There is no \"{}\" inside of the \"{}\".",
                            item, container_name
                        ),
                    )
                }
            } else {
                CmdResult::new(
                    false,
                    format!("You cannot take anything from the \"{}\".", container_name),
                )
            }
        } else {
            dont_have(container_name)
        }
    }

    pub fn wait() -> CmdResult {
        CmdResult::new(true, "Time passes...".to_owned())
    }
}
