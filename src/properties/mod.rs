use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Clone, Serialize, Deserialize)]
pub struct IsOpen(pub bool);

#[derive(Clone, Serialize, Deserialize)]
pub struct IsLocked(pub bool);
