use std::{fmt::Display, str::FromStr};

use crate::prelude::*;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

impl FromStr for NotEmptyText {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NotEmptyText::new(s.to_owned()).map_err(|error| error.to_string())
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
