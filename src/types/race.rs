use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Race {
    Human,
    Dwarf,
    Elf,
    Dragonborn,
}

impl Race {
    pub fn select_race(input: &str) -> Self {
        match input.chars().next() {
            Some('2') => Race::Dwarf,
            Some('3') => Race::Elf,
            Some('4') => Race::Dragonborn,
            _ => Race::Human,
        }
    }

    pub fn race_prompt() -> String {
        "Choose a race:\n  \
            1) Human\n  \
            2) Dwarf\n  \
            3) Elf\n  \
            4) Dragonborn\n\n"
            .to_owned()
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Human => "Human",
            Self::Dwarf => "Dwarf",
            Self::Elf => "Elf",
            Self::Dragonborn => "Dragonborn",
        };
        write!(f, "{}", s)
    }
}
