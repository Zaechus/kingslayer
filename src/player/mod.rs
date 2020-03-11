use serde::{Deserialize, Serialize};

use rayon::prelude::*;

use crate::{
    dice_roll,
    entity::{
        Entity,
        Item::{self, Armor, Weapon},
    },
    inventory::Inventory,
    types::{Action, CmdResult, CombatStatus, Items, Stats},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    hp: (i32, u32),
    xp: (u32, u32),
    in_combat: CombatStatus,
    lvl: u32,
    stats: Stats,
    main_hand: Option<Box<Item>>,
    armor: Option<Box<Item>>,
    inventory: Inventory,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hp: (13, 13),
            xp: (0, 1000),
            in_combat: CombatStatus::Resting,
            lvl: 1,
            stats: Stats::new(),
            main_hand: None,
            armor: None,
            inventory: Inventory::new(),
        }
    }
}

impl Player {
    fn deal_damage(&self, weapon_damage: u32) -> u32 {
        (weapon_damage as i32 + self.stats.strngth_mod()) as u32
    }

    fn default_damage(&self) -> u32 {
        dice_roll(1, 4)
    }

    pub fn attack(&mut self) -> Option<u32> {
        if let Some(weapon) = &self.main_hand {
            if let Weapon(ref weapon) = **weapon {
                Some(self.deal_damage(weapon.damage()))
            } else {
                Some(self.default_damage())
            }
        } else {
            None
        }
    }

    pub fn attack_with(&mut self, weapon_name: &str) -> Option<u32> {
        if let Some(weapon) = self.inventory.find(weapon_name) {
            if let Weapon(ref weapon) = **weapon {
                Some(self.deal_damage(weapon.damage()))
            } else {
                Some(self.default_damage())
            }
        } else if let Some(weapon) = &self.main_hand {
            if weapon_name == weapon.name() {
                if let Weapon(ref weapon) = **weapon {
                    Some(self.deal_damage(weapon.damage()))
                } else {
                    Some(self.default_damage())
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn close(&mut self, item_name: &str) -> Option<CmdResult> {
        self.inventory.close(item_name)
    }

    pub fn disengage_combat(&mut self) {
        self.in_combat = CombatStatus::Resting;
    }

    fn set_armor(&mut self, armor_name: &str, item: Box<Item>) -> CmdResult {
        if let Armor(_) = *item {
            // move old armor back to inventory
            if let Some(armor) = self.armor.take() {
                self.take(armor_name, Some(armor));
            }
            self.armor = Some(item);
            CmdResult::new(Action::Active, "Donned.".to_owned())
        } else {
            self.inventory.push(item);
            CmdResult::new(
                Action::Passive,
                format!(
                    "You cannot put on the \"{}\", which isn't armor.",
                    armor_name
                ),
            )
        }
    }

    pub fn don_armor(&mut self, armor_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.position(armor_name) {
            let item = self.inventory.remove(item);
            self.set_armor(armor_name, item)
        } else if let Some(item) = self.inventory.find_similar_item(armor_name) {
            let item = self.inventory.remove(item);
            self.set_armor(&armor_name, item)
        } else {
            CmdResult::dont_have(armor_name)
        }
    }

    pub fn engage_combat(&mut self) {
        self.in_combat = CombatStatus::InCombat;
    }

    fn set_equipped(&mut self, item_name: &str, item: Box<Item>) -> CmdResult {
        // move old main hand back to inventory
        match &*item {
            Armor(_) => {
                self.take(item_name, Some(item));
                self.don_armor(item_name);
                CmdResult::new(Action::Active, "Donned.".to_owned())
            }
            Weapon(_) => {
                if let Some(weapon) = self.main_hand.take() {
                    self.take(&weapon.name().to_owned(), Some(weapon));
                }
                self.main_hand = Some(item);
                CmdResult::new(Action::Active, "Equipped.".to_owned())
            }
            _ => {
                self.inventory.push(item);
                CmdResult::new(
                    Action::Active,
                    format!("The {} is neither armor nor weapon.", item_name),
                )
            }
        }
    }

    // equip an Item into main_hand to simplify fighting
    pub fn equip(&mut self, weapon_name: &str) -> CmdResult {
        if let Some(item) = self.inventory.position(weapon_name) {
            let item = self.inventory.remove(item);
            self.set_equipped(weapon_name, item)
        } else if let Some(item) = self.inventory.find_similar_item(weapon_name) {
            let item = self.inventory.remove(item);
            self.set_equipped(weapon_name, item)
        } else {
            CmdResult::dont_have(weapon_name)
        }
    }

    pub fn gain_xp(&mut self, gained: u32) {
        self.xp.0 += gained;
    }

    pub fn has(&self, name: &str) -> bool {
        self.inventory.has(name)
    }

    pub const fn hp(&self) -> i32 {
        self.hp.0
    }

    pub const fn hp_cap(&self) -> u32 {
        self.hp.1
    }

    pub fn increase_ability_score(&mut self, ability_score: &str) -> CmdResult {
        self.stats.increase_ability_score(ability_score)
    }

    pub fn insert_into(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        self.inventory.insert_into(item_name, container_name)
    }

    pub fn info(&self) -> CmdResult {
        CmdResult::new(
            Action::Passive,
            format!(
                "Level: {}\
                 \nHP: ({} / {})\
                 \nAC: {}\
                 \nXP: ({} / {})\
                 \n{}",
                self.lvl,
                self.hp(),
                self.hp_cap(),
                self.ac(),
                self.xp.0,
                self.xp.1,
                self.stats.print_stats()
            ),
        )
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if name == "me" || name == "self" || name == "myself" {
            Some(self.info())
        } else if let Some(item) = self.inventory.find(name) {
            Some(CmdResult::new(Action::Active, item.inspect().to_owned()))
        } else if let Some(item) = &self.main_hand {
            if item.name() == name {
                Some(CmdResult::new(Action::Active, item.inspect().to_owned()))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub const fn is_alive(&self) -> bool {
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

    pub const fn main_hand(&self) -> &Option<Box<Item>> {
        &self.main_hand
    }

    pub fn open(&mut self, item_name: &str) -> Option<CmdResult> {
        self.inventory.open(item_name)
    }

    pub fn print_inventory(&self) -> CmdResult {
        let mut items_carried = String::with_capacity(25);
        if let Some(weapon) = &self.main_hand {
            items_carried.push_str(&format!("Main hand: {}\n", weapon.name()));
        }
        if let Some(armor) = &self.armor {
            items_carried.push_str(&format!("Armor: {}\n", armor.name()));
        }
        items_carried.push_str(&self.inventory.print());
        items_carried.shrink_to_fit();
        CmdResult::new(Action::Active, items_carried)
    }

    fn remove_main_hand(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.main_hand.take() {
            let similar = if cfg!(target_arch = "wasm32") {
                item.name().split_whitespace().any(|word| word == name)
            } else {
                item.name().par_split_whitespace().any(|word| word == name)
            };
            if item.name() == name || similar {
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
            let similar = if cfg!(target_arch = "wasm32") {
                item.name().split_whitespace().any(|word| word == name)
            } else {
                item.name().par_split_whitespace().any(|word| word == name)
            };
            if item.name() == name || similar {
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
        if let Some(item) = self.inventory.remove_item(name) {
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
            if let CombatStatus::Resting = self.in_combat {
                let regained_hp = dice_roll(1, 6);
                let new_hp = self.hp() + regained_hp as i32;
                if new_hp < self.hp_cap() as i32 {
                    self.hp = (new_hp, self.hp_cap());
                } else {
                    self.hp = (self.hp_cap() as i32, self.hp_cap());
                }
                CmdResult::new(
                    Action::Active,
                    format!(
                        "You regained {} HP for a total of ({} / {}) HP.",
                        regained_hp,
                        self.hp(),
                        self.hp_cap()
                    ),
                )
            } else {
                CmdResult::new(
                    Action::Passive,
                    "You cannot rest while in combat.".to_owned(),
                )
            }
        } else {
            CmdResult::new(Action::Passive, "You already have full health.".to_owned())
        }
    }

    fn ac(&self) -> i32 {
        if let Some(armor) = &self.armor {
            if let Armor(ref armor) = **armor {
                armor.ac() as i32 + self.stats.dex_mod()
            } else {
                10 + self.stats.dex_mod()
            }
        } else {
            10 + self.stats.dex_mod()
        }
    }

    pub fn take_damage(&mut self, enemy_name: &str, damage: u32) -> String {
        if dice_roll(1, 20) as i32 >= self.ac() {
            self.hp = (self.hp.0 - damage as i32, self.hp.1);
            format!(
                "\nThe {} hit you for {} damage. You have {} HP left.",
                enemy_name,
                damage,
                self.hp()
            )
        } else {
            match dice_roll(1, 3) {
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
        self.inventory.take(name, item)
    }

    pub fn take_all(&mut self, items: Items) -> CmdResult {
        self.inventory.take_all(items)
    }

    pub fn take_back(&mut self, item: Box<Item>) {
        self.inventory.push(item);
    }

    pub fn take_from_self(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        self.inventory.take_from_self(item_name, container_name)
    }

    pub fn take_item_from(&mut self, item: Result<Box<Item>, CmdResult>) -> CmdResult {
        self.inventory.take_item_from(item)
    }

    pub fn wait() -> CmdResult {
        CmdResult::new(Action::Active, "Time passes...".to_owned())
    }
}
