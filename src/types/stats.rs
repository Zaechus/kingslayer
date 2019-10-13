use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub pts: u32,
    pub strngth: u32,
    pub dex: u32,
    pub con: u32,
    pub int: u32,
    pub wis: u32,
    pub cha: u32,
}

impl Stats {
    pub fn new() -> Self {
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
            "\
             Strength:     {}\
             \nDexterity:    {}\
             \nConstitution: {}\
             \nIntellect:    {}\
             \nWisdom:       {}\
             \nCharisma:     {} ",
            self.strngth, self.dex, self.con, self.int, self.wis, self.cha
        )
    }

    pub fn strngth_mod(&self) -> u32 {
        (f64::from(self.strngth - 10) / 2.0).floor() as u32
    }
    pub fn dex_mod(&self) -> u32 {
        (f64::from(self.dex - 10) / 2.0).floor() as u32
    }
    // pub fn con_mod(&self) -> u32 {
    //     (f64::from(self.con - 10) / 2.0).floor() as u32
    // }
    // pub fn int_mod(&self) -> u32 {
    //     (f64::from(self.int - 10) / 2.0).floor() as u32
    // }
    // pub fn wis_mod(&self) -> u32 {
    //     (f64::from(self.wis - 10) / 2.0).floor() as u32
    // }
    // pub fn cha_mod(&self) -> u32 {
    //     (f64::from(self.cha - 10) / 2.0).floor() as u32
    // }
}
