use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Class {
    Fighter,
    Wizard,
    Rogue,
}

impl Class {
    pub fn to_string(&self) -> String {
        match self {
            Self::Fighter => "Fighter".to_string(),
            Self::Wizard => "Wizard".to_string(),
            Self::Rogue => "Rogue".to_string(),
        }
    }

    pub fn select_class(input: &str) -> Self {
        match input.chars().next() {
            Some('2') => Class::Rogue,
            Some('3') => Class::Wizard,
            _ => Class::Fighter,
        }
    }
}
