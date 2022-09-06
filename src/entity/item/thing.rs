use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Thing {
    name: String,
    desc: String,
    inspect: String,
}

impl Entity for Thing {
    fn name(&self) -> &str {
        &self.name
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn inspect(&self) -> &str {
        &self.inspect
    }
}
