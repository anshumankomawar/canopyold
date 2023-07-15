use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Link {
    pub url: String,
    pub label: String,
}
