use serde::{Deserialize, Serialize};

use rayon::prelude::*;

use crate::{
    dice_roll,
    entity::{
        Entity,
        Item::{self, Armor, Weapon},
    },
    inventory::Inventory,
    types::{Action, Attack, Class, CmdResult, CombatStatus, Items, Race, Stats},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    lvl: u32,
    race: Race,
    class: Class,
    hp: (i32, u32),
    xp: (u32, u32),
    in_combat: CombatStatus,
    stats: Stats,
    main_hand: Option<Box<Item>>,
    armor: Option<Box<Item>>,
    inventory: Inventory,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            lvl: 1,
            race: Race::Human,
            class: Class::Warrior,
            hp: (13, 13),
            xp: (0, 1000),
            in_combat: CombatStatus::Resting,
            stats: Stats::new(),
            main_hand: None,
            armor: None,
            inventory: Inventory::new(),
        }
    }
}

impl Player {
    pub fn set_race(&mut self, race: Race) {
        self.race = race;
    }

    pub fn set_class(&mut self, class: Class) {
        self.class = class;
    }

    fn deal_damage(&self, weapon_damage: u32) -> u32 {
        (weapon_damage as i32 + self.stats.strngth_mod()) as u32
    }

    fn default_damage(&self) -> u32 {
        dice_roll(1, 4)
    }

    pub fn attack_main(&mut self) -> Attack {
        if let Some(weapon) = &self.main_hand {
            if let Weapon(ref weapon) = **weapon {
                Attack::new(weapon.name(), Some(self.deal_damage(weapon.damage())))
            } else {
                Attack::new(weapon.name(), Some(self.default_damage()))
            }
        } else {
            Attack::default()
        }
    }

    pub fn attack_with(&mut self, weapon_name: &str) -> Attack {
        if let Some(weapon) = self.inventory.find_item(weapon_name) {
            if let Weapon(ref weapon) = weapon {
                Attack::new(weapon_name, Some(self.deal_damage(weapon.damage())))
            } else {
                Attack::new(weapon_name, Some(self.default_damage()))
            }
        } else if self.is_main_hand(weapon_name) {
            if let Some(weapon) = &self.main_hand {
                if let Weapon(ref weapon) = **weapon {
                    Attack::new(weapon_name, Some(self.deal_damage(weapon.damage())))
                } else {
                    Attack::new(weapon_name, Some(self.default_damage()))
                }
            } else {
                Attack::new(weapon_name, None)
            }
        } else {
            Attack::new(weapon_name, None)
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
            CmdResult::new(Action::Active, "Donned.")
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
        if let Some(item) = self.inventory.remove_item(armor_name) {
            self.set_armor(armor_name, item)
        } else {
            CmdResult::dont_have(armor_name)
        }
    }

    pub fn engage_combat(&mut self) {
        self.in_combat = CombatStatus::InCombat;
    }

    fn set_equipped(&mut self, item_name: &str, item: Box<Item>) -> CmdResult {
        match *item {
            Armor(_) => {
                self.take(item_name, Some(item));
                self.don_armor(item_name);
                CmdResult::new(Action::Active, "Donned.")
            }
            Weapon(_) => {
                if let Some(weapon) = self.main_hand.take() {
                    self.take("", Some(weapon));
                }
                self.main_hand = Some(item);
                CmdResult::new(Action::Active, "Equipped.")
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
        if let Some(item) = self.inventory.remove_item(weapon_name) {
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

    pub fn in_combat(&self) -> bool {
        match self.in_combat {
            CombatStatus::Resting => false,
            CombatStatus::InCombat => true,
        }
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
                "Level {} {} {}\
                 \nHP: ({} / {})\
                 \nAC: {}\
                 \nXP: ({} / {})\
                 \n{}",
                self.lvl,
                self.race,
                self.class,
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
        } else if let Some(item) = self.inventory.find_item(name) {
            Some(CmdResult::new(Action::Active, item.inspect()))
        } else if let Some(item) = &self.main_hand {
            if item.name() == name {
                Some(CmdResult::new(Action::Active, item.inspect()))
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
            self.xp.1 = (1800 * (self.lvl as i32 - 2).pow(2) + 1000) as u32;
            self.lvl += 1;
            self.stats.pts += self.lvl + 3;
            format!("\n\nYou advanced to level {}!", self.lvl)
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
        CmdResult::new(
            Action::Active,
            format!(
                "{}{}{}",
                if let Some(weapon) = &self.main_hand {
                    format!("Main hand: {}\n", weapon.name())
                } else {
                    String::new()
                },
                if let Some(armor) = &self.armor {
                    format!("Armor: {}\n", armor.name())
                } else {
                    String::new()
                },
                self.inventory.print()
            ),
        )
    }

    fn is_main_hand(&self, name: &str) -> bool {
        if let Some(main_hand) = &self.main_hand {
            let main_hand = main_hand
                .name()
                .par_split_whitespace()
                .collect::<Vec<&str>>();

            name.par_split_whitespace()
                .all(|ref word| main_hand.contains(word))
        } else {
            false
        }
    }

    fn remove_main_hand(&mut self, name: &str) -> Option<Box<Item>> {
        if self.is_main_hand(name) {
            self.main_hand.take()
        } else {
            None
        }
    }

    fn is_armor(&self, name: &str) -> bool {
        if let Some(armor) = &self.armor {
            let armor = armor.name().par_split_whitespace().collect::<Vec<&str>>();

            name.par_split_whitespace()
                .all(|ref word| armor.contains(word))
        } else {
            false
        }
    }

    fn remove_armor(&mut self, name: &str) -> Option<Box<Item>> {
        if self.is_armor(name) {
            self.armor.take()
        } else {
            None
        }
    }

    // remove an item from inventory and into the current Room
    pub fn remove(&mut self, item_name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.inventory.remove_item(item_name) {
            Some(item)
        } else if let Some(item) = self.remove_main_hand(item_name) {
            Some(item)
        } else {
            self.remove_armor(item_name)
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
                CmdResult::new(Action::Passive, "You cannot rest while in combat.")
            }
        } else {
            CmdResult::new(Action::Passive, "You already have full health.")
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
        CmdResult::new(Action::Active, "Time passes...")
    }
}
