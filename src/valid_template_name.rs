use std::{fmt::Display, str::FromStr};

use crate::prelude::*;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ValidTemplateName(String);

impl ValidTemplateName {
    pub fn new(to_validate: String) -> AppResult<Self> {
        if to_validate.trim().is_empty() {
            Err(anyhow!("Text must no be empty or only whitespce"))
        } else if to_validate.contains(std::path::MAIN_SEPARATOR_STR) {
            Err(anyhow!(
                "Text must no contain any path seperator aka slaches."
            ))
        } else {
            Ok(Self(to_validate))
        }
    }
    pub fn clone_from(input: &str) -> AppResult<Self> {
        Self::new(input.to_string())
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
        assert_case("");
        assert_case("  ");
        fn assert_case(input: &str) {
            let actual = ValidTemplateName::clone_from(input);
            assert!(actual.is_err());
        }
    }
    #[test]
    fn trim_and_accept() {
        assert_case("a", "a");
        assert_case(" a  ", " a  ");
        fn assert_case(input: &str, expected: &str) {
            let actual = ValidTemplateName::clone_from(input).unwrap();
            assert_eq!(expected, actual.as_ref());
        }
    }
    #[test]
    fn reject_path_sep() {
        assert_case(&format!("{}", std::path::MAIN_SEPARATOR_STR));
        assert_case(&format!("aa{}bb", std::path::MAIN_SEPARATOR_STR));
        fn assert_case(input: &str) {
            let actual = ValidTemplateName::clone_from(input);
            assert!(actual.is_err());
        }
    }
}
