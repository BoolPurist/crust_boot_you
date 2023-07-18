use serde::{Deserialize, Serialize};

use crate::NotEmptyText;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct TemplateMeta {
    name: NotEmptyText,
}

impl TemplateMeta {
    pub fn new(name: NotEmptyText) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
