use serde::{Deserialize, Serialize};

use super::{Action, CmdResult};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    pub pts: u32,
    pub strngth: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}

impl Stats {
    pub const fn new() -> Self {
        Self {
            pts: 4,
            strngth: 14,
            dex: 13,
            con: 15,
            int: 12,
            wis: 10,
            cha: 8,
        }
    }

    pub fn print_stats(&self) -> String {
        format!(
            "Stat points: {}\
             \n  Strength:     {} ({})\
             \n  Dexterity:    {} ({})\
             \n  Constitution: {} ({})\
             \n  Intellect:    {} ({})\
             \n  Wisdom:       {} ({})\
             \n  Charisma:     {} ({}) ",
            self.pts,
            self.strngth,
            self.strngth_mod(),
            self.dex,
            self.dex_mod(),
            self.con,
            self.con_mod(),
            self.int,
            self.int_mod(),
            self.wis,
            self.wis_mod(),
            self.cha,
            self.cha_mod(),
        )
    }

    pub fn increase_ability_score(&mut self, ability_score: &str) -> CmdResult {
        if self.pts > 0 {
            match &ability_score[0..3] {
                "str" | "dex" | "con" | "int" | "wis" | "cha" => {
                    self.pts -= 1;
                    match &ability_score[0..3] {
                        "str" => self.strngth += 1,
                        "dex" => self.dex += 1,
                        "con" => self.con += 1,
                        "int" => self.int += 1,
                        "wis" => self.wis += 1,
                        "cha" => self.cha += 1,
                        _ => (),
                    }
                    CmdResult::new(Action::Active, "Ability score increased by one.")
                }
                _ => CmdResult::new(
                    Action::Passive,
                    format!("\"{}\" is not a valid ability score.", ability_score),
                ),
            }
        } else {
            CmdResult::new(Action::Passive, "You do not have any stat points.")
        }
    }

    pub fn strngth_mod(&self) -> i32 {
        (f64::from(self.strngth - 10) / 2.0).floor() as i32
    }
    pub fn dex_mod(&self) -> i32 {
        (f64::from(self.dex - 10) / 2.0).floor() as i32
    }
    pub fn con_mod(&self) -> i32 {
        (f64::from(self.con - 10) / 2.0).floor() as i32
    }
    pub fn int_mod(&self) -> i32 {
        (f64::from(self.int - 10) / 2.0).floor() as i32
    }
    pub fn wis_mod(&self) -> i32 {
        (f64::from(self.wis - 10) / 2.0).floor() as i32
    }
    pub fn cha_mod(&self) -> i32 {
        (f64::from(self.cha - 10) / 2.0).floor() as i32
    }
}
