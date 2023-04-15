use crate::types::permissions::Permissions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct User {
    #[serde(default)]
    pub(crate) uuid: String,
    #[serde(default)]
    pub(crate) username: String,
    #[serde(default)]
    pub(crate) discriminator: String,
    #[serde(default)]
    pub(crate) faction: String,
    #[serde(default)]
    pub(crate) permissions: Vec<Permissions>,
}

impl User {
    pub fn permitted(&self, permission: Permissions) -> bool {
        self.permissions.contains(&permission) || self.permissions.contains(&Permissions::Leader)
    }
}