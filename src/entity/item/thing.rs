use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Thing {
    name: String,
    desc: String,
    inspect: String,
}

impl Thing {
    pub(super) fn _new<S: Into<String>>(name: S, desc: S, inspect: S) -> Self {
        Self {
            name: name.into(),
            desc: desc.into(),
            inspect: inspect.into(),
        }
    }
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
