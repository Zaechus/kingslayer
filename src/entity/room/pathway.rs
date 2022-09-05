use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Pathway {
    pub(super) directions: Vec<String>,
    target: String,
    desc: String,
    inspect: String,
}

impl Pathway {
    pub(crate) fn target(&self) -> &str {
        &self.target
    }
}
