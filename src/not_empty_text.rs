use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::*;
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NotEmptyText(String);

impl NotEmptyText {
    pub fn new(must_not_be_empty: String) -> AppResult<Self> {
        if must_not_be_empty.trim().is_empty() {
            Err(anyhow!("Text is empty or only whitespce"))
        } else {
            Ok(Self(must_not_be_empty))
        }
    }

    pub fn new_clone_panic(must_not_be_empty: &str) -> Self {
        Self::new(must_not_be_empty.to_owned()).unwrap()
    }
}

impl AsRef<str> for NotEmptyText {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl Display for NotEmptyText {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <String as Display>::fmt(&self.0, f)
    }
}
