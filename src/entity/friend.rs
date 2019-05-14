use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Friend {
    hp: i32,
    name: String,
    desc: String,
    inspection: String,
}

impl Friend {}
