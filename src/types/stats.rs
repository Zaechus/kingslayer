use serde_derive::{Deserialize, Serialize};

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
}
