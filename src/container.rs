use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) enum Container {
    Open,
    Closed,
    True,
    #[default]
    False,
}
