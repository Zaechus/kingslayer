use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Class {
    Fighter,
    Wizard,
    Rogue,
}

impl Class {
    pub fn select_class(input: &str) -> Self {
        match input.chars().next() {
            Some('2') => Class::Rogue,
            Some('3') => Class::Wizard,
            _ => Class::Fighter,
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Fighter => "Fighter",
            Self::Wizard => "Wizard",
            Self::Rogue => "Rogue",
        };
        write!(f, "{}", s.to_owned())
    }
}
