use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CombatStatus {
    InCombat,
    Resting,
}

impl Default for CombatStatus {
    fn default() -> Self {
        CombatStatus::Resting
    }
}
