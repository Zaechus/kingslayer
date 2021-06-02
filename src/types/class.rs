use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Class {
    Warrior,
    Assassin,
    Mage,
    Cleric,
    Ranger,
    Necromancer,
    Shaman,
    Druid,
    Elementalist,
    Monk,
    Templar,
}

impl Class {
    pub fn select_class(input: &str) -> Self {
        match input {
            "2" => Class::Assassin,
            "3" => Class::Mage,
            "4" => Class::Cleric,
            "5" => Class::Ranger,
            "6" => Class::Necromancer,
            "7" => Class::Shaman,
            "8" => Class::Druid,
            "9" => Class::Elementalist,
            "10" => Class::Monk,
            "11" => Class::Templar,
            _ => Class::Warrior,
        }
    }

    pub fn class_prompt() -> String {
        "Choose a class:\n  \
            1) Warrior\n  \
            2) Assassin\n  \
            3) Mage\n  \
            4) Cleric\n  \
            5) Ranger\n  \
            6) Necromancer\n  \
            7) Shaman\n  \
            8) Druid\n  \
            9) Elementalist\n  \
            10) Monk\n  \
            11) Templar\n\n"
            .to_owned()
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Warrior => "Warrior",
            Self::Assassin => "Assassin",
            Self::Mage => "Mage",
            Self::Cleric => "Cleric",
            Self::Ranger => "Ranger",
            Self::Necromancer => "Necromancer",
            Self::Shaman => "Shaman",
            Self::Druid => "Druid",
            Self::Elementalist => "Elementalist",
            Self::Monk => "Monk",
            Self::Templar => "Templar",
        };
        write!(f, "{}", s)
    }
}
