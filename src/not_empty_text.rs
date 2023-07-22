use std::{fmt::Display, str::FromStr};

use crate::prelude::*;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ValidTemplateName(String);

impl ValidTemplateName {
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

impl FromStr for ValidTemplateName {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ValidTemplateName::new(s.to_owned()).map_err(|error| error.to_string())
    }
}

impl AsRef<str> for ValidTemplateName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl Display for ValidTemplateName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <String as Display>::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn reject_empty_or_only_white_space() {
        todo!()
    }
}
