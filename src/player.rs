use rand::Rng;

use serde::{Deserialize, Serialize};

use crate::{
    entity::{
        Closeable, Entity,
        Item::{self, Armor, Container, Weapon},
    },
    response::{dont_have, no_item_here, not_container},
    types::Stats,
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
            if let Armor(armor) = &**armor {
                armor.ac() + (f64::from(self.stats.dex - 10) / 2.0).floor() as u32
            } else {
                10 + (f64::from(self.stats.dex - 10) / 2.0).floor() as u32
            }
        } else {
            10 + (f64::from(self.stats.dex - 10) / 2.0).floor() as u32
        }
    }

    pub fn attack(&mut self) -> Option<u32> {
        if let Some(weapon) = &self.main_hand {
            if let Weapon(weapon) = &**weapon {
                self.in_combat = true;
                Some(self.deal_damage(weapon.damage()))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn attack_with(&mut self, weapon_name: &str) -> Option<u32> {
        if let Some(weapon) = self.inventory.get(weapon_name) {
            if let Weapon(weapon) = &**weapon {
                self.in_combat = true;
                Some(self.deal_damage(weapon.damage()))
            } else {
                None
            }
        } else if let Some(weapon) = &self.main_hand {
            if weapon_name == weapon.name() {
                if let Weapon(weapon) = &**weapon {
                    self.in_combat = true;
                    Some(self.deal_damage(weapon.damage()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn close(&mut self, item_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.get_mut(item_name) {
            if let Container(item) = &mut **item {
                if item.is_closed() {
                    CmdResult::new(false, format!("The {} is already closed.", item_name))
                } else {
                    item.close();
                    CmdResult::new(true, "Closed.".to_owned())
                }
            } else {
                not_container(item_name)
            }
        } else {
            no_item_here(item_name)
        }
    }

    fn deal_damage(&self, weapon_damage: u32) -> u32 {
        weapon_damage + (f64::from(self.stats.strngth - 10) / 2.0).floor() as u32
    }

    pub fn disengage_combat(&mut self) {
        self.in_combat = false;
    }

    fn set_armor(&mut self, armor_name: &str, item: Box<Item>) -> CmdResult {
        if let Armor(_) = &*item {
            // move old armor back to inventory
            if let Some(armor) = self.armor.take() {
                self.take(armor_name, Some(armor));
            }
            self.armor = Some(item);
            CmdResult::new(
                    true,
                    "Donned.\n(You can remove armor with \"drop\" or by donning a different set or armor)"
                        .to_owned()
                )
        } else {
            self.inventory.insert(item.name().to_owned(), item);
            CmdResult::new(
                false,
                format!(
                    "You cannot put on the \"{}\", which isn't armor.",
                    armor_name
                ),
            )
        }
    }

    pub fn don_armor(&mut self, armor_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.remove(armor_name) {
            self.set_armor(armor_name, item)
        } else {
            let similar_name = if let Some(similar_name) = self
                .inventory
                .keys()
                .find(|key| key.split_whitespace().any(|word| word == armor_name))
            {
                similar_name.clone()
            } else {
                return dont_have(armor_name);
            };
            if let Some(item) = self.inventory.remove(&similar_name) {
                self.set_armor(&similar_name, item)
            } else {
                dont_have(armor_name)
            }
        }
    }

    pub fn engage_combat(&mut self) {
        self.in_combat = true;
    }

    fn set_equipped(&mut self, item_name: &str, item: Box<Item>) -> CmdResult {
        // move old main hand back to inventory
        if let Some(weapon) = self.main_hand.take() {
            self.take(&weapon.name().to_owned(), Some(weapon));
        }
        match &*item {
            Armor(_) => {
                self.take(item_name, Some(item));
                self.don_armor(item_name);
                CmdResult::new(true, "Donned.".to_owned())
            }
            Weapon(_) => {
                self.main_hand = Some(item);
                CmdResult::new(
                    true,
                    "Equipped.\n(You can unequip items with \"drop\" or by equipping a different item)"
                        .to_owned(),
                )
            }
            _ => CmdResult::new(
                false,
                format!("The {} is neither armor nor weapon.", item_name),
            ),
        }
    }

    // equip an Item into main_hand to simplify fighting
    pub fn equip(&mut self, weapon_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.remove(weapon_name) {
            self.set_equipped(weapon_name, item)
        } else {
            let similar_name = if let Some(similar_name) = self
                .inventory
                .keys()
                .find(|key| key.split_whitespace().any(|word| word == weapon_name))
            {
                similar_name.clone()
            } else {
                return dont_have(weapon_name);
            };
            if let Some(item) = self.inventory.remove(&similar_name) {
                self.set_equipped(&similar_name, item)
            } else {
                dont_have(weapon_name)
            }
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
                    match &ability_score[0..3] {
                        "str" => self.stats.strngth += 1,
                        "dex" => self.stats.dex += 1,
                        "con" => self.stats.con += 1,
                        "int" => self.stats.int += 1,
                        "wis" => self.stats.wis += 1,
                        "cha" => self.stats.cha += 1,
                        _ => (),
                    }
                    CmdResult::new(true, "Ability score increased by one.".to_owned())
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
        if let Some(item) = self.inventory.remove(item_name) {
            if let Some(container) = self.inventory.get_mut(container_name) {
                if let Container(container) = &mut **container {
                    if container.is_closed() {
                        self.inventory.insert(item_name.to_owned(), item);
                        CmdResult::new(true, format!("The {} is closed.", container_name))
                    } else {
                        container
                            .contents_mut()
                            .insert(item.name().to_owned(), item);
                        CmdResult::new(true, "Placed.".to_owned())
                    }
                } else {
                    self.inventory.insert(item_name.to_owned(), item);
                    not_container(container_name)
                }
            } else {
                self.inventory.insert(item_name.to_owned(), item);
                dont_have(container_name)
            }
        } else {
            CmdResult::new(false, format!("You do not have the \"{}\".", item_name))
        }
    }

    pub fn info(&self) -> CmdResult {
        CmdResult::new(
            false,
            format!(
                "Level: {}\
                \nHP: ({} / {})\
                \nAC: {}\
                \nXP: ({} / {})\
                \nStat points: {}\
                
                \n\n{}",
                self.lvl,
                self.hp(),
                self.hp_cap(),
                self.ac(),
                self.xp.0,
                self.xp.1,
                self.stats.pts,
                self.stats.print_stats()
            ),
        )
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if name == "me" || name == "self" || name == "myself" {
            Some(self.info())
        } else if let Some(item) = self.inventory.get(name) {
            Some(CmdResult::new(true, item.inspect().to_owned()))
        } else if let Some(item) = &self.main_hand {
            Some(CmdResult::new(true, item.inspect().to_owned()))
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
            if let Container(item) = &mut **item {
                if item.is_closed() {
                    item.open();
                    CmdResult::new(true, "Opened.".to_owned())
                } else {
                    CmdResult::new(false, format!("The {} is already opened.", item_name))
                }
            } else {
                not_container(item_name)
            }
        } else {
            no_item_here(item_name)
        }
    }

    pub fn print_inventory(&self) -> CmdResult {
        let mut items_carried = String::new();
        if let Some(weapon) = &self.main_hand {
            items_carried.push_str(&format!("Main hand: {}\n", weapon.long_name()));
        }
        if let Some(armor) = &self.armor {
            items_carried.push_str(&format!("Armor: {}\n", armor.long_name()));
        }
        if self.inventory.is_empty() {
            items_carried.push_str("Your inventory is empty.");
        } else {
            items_carried.push_str("You are carrying:");
            for item in self.inventory.values() {
                items_carried = format!("{}\n  {}", items_carried, item.long_name());
            }
        }
        CmdResult::new(true, items_carried)
    }

    fn remove_item(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.inventory.remove(name) {
            Some(item)
        } else {
            let similar_name = if let Some(similar_name) = self
                .inventory
                .keys()
                .find(|key| key.split_whitespace().any(|word| word == name))
            {
                similar_name.clone()
            } else {
                return None;
            };
            self.inventory.remove(&similar_name)
        }
    }

    fn remove_main_hand(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.main_hand.take() {
            if item.name() == name || item.name().split_whitespace().any(|word| word == name) {
                Some(item)
            } else {
                self.main_hand = Some(item);
                None
            }
        } else {
            None
        }
    }

    fn remove_armor(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.armor.take() {
            if item.name() == name || item.name().split_whitespace().any(|word| word == name) {
                Some(item)
            } else {
                self.armor = Some(item);
                None
            }
        } else {
            None
        }
    }

    // remove an item from inventory and into the current Room
    pub fn remove(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.remove_item(name) {
            Some(item)
        } else if let Some(item) = self.remove_main_hand(name) {
            Some(item)
        } else if let Some(item) = self.remove_armor(name) {
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
            if let Weapon(_) = *obj {
                res.push_str("\n(You can equip weapons with \"equip\" or \"draw\")");
            }
            self.inventory.insert(obj.name().to_owned(), obj);
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
    pub fn take_from(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        if let Some(container) = self.inventory.get_mut(container_name) {
            if let Container(container) = &mut **container {
                if let Some(item) = container.contents_mut().remove(item_name) {
                    self.inventory.insert(item.name().to_owned(), item);
                    CmdResult::new(true, "Taken.".to_owned())
                } else {
                    CmdResult::new(
                        true,
                        format!(
                            "There is no \"{}\" inside of the \"{}\".",
                            item_name, container_name
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
