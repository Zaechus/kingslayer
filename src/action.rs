use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum Action {
    Again,
    Attack(String, String),
    Break(String),
    Burn(String, String),
    Clarify(String),
    Climb,
    Close(String),
    Drop(String),
    Eat(String),
    Examine(String),
    Hello,
    Help,
    Inventory,
    Look,
    Move(String),
    NoVerb,
    Open(String),
    Put(String, String),
    Sleep,
    Take(String),
    Unknown(String),
    Walk(String),
    Wear(String),
    Where(String),
}

impl Default for Action {
    fn default() -> Self {
        Self::Look
    }
}

impl Action {
    pub(crate) fn what_do(s: &str) -> Self {
        Self::Clarify(format!("What do you want to {}?", s))
    }
}
