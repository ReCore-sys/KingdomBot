use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub(crate) enum Permissions {
    NoMoneyCap,
    MoveTroops,
    Build,
    War,
    Leader,
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}