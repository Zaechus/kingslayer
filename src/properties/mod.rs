use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize)]
pub struct IsOpen(pub bool);

#[derive(Serialize, Deserialize)]
pub struct IsLocked(pub bool);
