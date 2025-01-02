use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum CombatStatus {
    InCombat,
    #[default]
    Resting,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum EnemyStatus {
    Angry,
    #[default]
    Distracted,
    Asleep,
}
