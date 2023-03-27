use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct User {
    pub(crate) uuid: String,
    pub(crate) username: String,
    pub(crate) discriminator: String,
    pub(crate) faction: String,
}