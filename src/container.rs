use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Container {
    Open,
    Closed,
    True,
    False,
}

impl Default for Container {
    fn default() -> Self {
        Self::False
    }
}
