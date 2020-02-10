use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum CombatStatus {
    InCombat,
    Resting,
}

impl Default for CombatStatus {
    fn default() -> Self {
        CombatStatus::Resting
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum EnemyStatus {
    Angry,
    Distracted,
    Asleep,
}

impl Default for EnemyStatus {
    fn default() -> Self {
        EnemyStatus::Distracted
    }
}
