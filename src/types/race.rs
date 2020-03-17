use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Race {
    Human,
    Dwarf,
    Elf,
}

impl Race {
    pub fn to_string(&self) -> String {
        match self {
            Self::Human => "Human".to_string(),
            Self::Dwarf => "Dwarf".to_string(),
            Self::Elf => "Elf".to_string(),
        }
    }

    pub fn select_race(input: &str) -> Self {
        match input.chars().next() {
            Some('2') => Race::Dwarf,
            Some('3') => Race::Elf,
            _ => Race::Human,
        }
    }
}
