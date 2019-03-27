use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Friend {
    hp: i32,
    name: String,
    desc: String,
    inspection: String,
}

impl Friend {
    // pub fn hp(&self) -> i32 {
    //     self.hp
    // }

    // pub fn name(&self) -> &String {
    //     &self.name
    // }

    // pub fn desc(&self) -> &String {
    //     &self.desc
    // }

    // pub fn inspection(&self) -> &String {
    //     &self.inspection
    // }
}
