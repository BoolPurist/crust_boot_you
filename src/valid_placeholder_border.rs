use std::str::FromStr;

use crate::prelude::*;
use derive_more::{AsRef, Deref, Display};

#[derive(Debug, AsRef, Deref, Display, Into, Clone, PartialEq, Eq)]
pub struct ValidPlaceholderBorder(String);

impl ValidPlaceholderBorder {
    pub fn new(to_validate: String) -> AppResult<Self> {
        let (trimmed_len, to_validate_len) = (to_validate.trim().len(), to_validate.len());
        ensure!(
            trimmed_len == to_validate_len, 
            "Border of a placeholder must not contain any non-visible characters to the left or right, aka whitespace"
        );
        Ok(Self(to_validate))
    }
}
impl FromStr for ValidPlaceholderBorder {
    type Err = AppError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ValidPlaceholderBorder::new(s.to_string())
    }
}
#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn accept_valid_placeholder_borders() {
         
        assert_case("aa");
        assert_case("a aaa a");
        assert_case("z zzz z zz z z");
        assert_case("a \t \n a");
        fn assert_case(input: &str) {
            let actual: ValidPlaceholderBorder = input.parse().unwrap();
            
            assert_eq!(input, actual.as_str());
        }
    }
    #[test]
    fn deny_invalid_placeholder_borders() {
         
        assert_case("  aa");
        assert_case("a aaa a   ");
        assert_case("\tz zzz z zz z z");
        assert_case("a \t \n a\n");
        fn assert_case(input: &str) {
            let _ = input.parse::<ValidPlaceholderBorder>().unwrap_err();
        }
    }
}
