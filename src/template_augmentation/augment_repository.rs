use std::collections::HashMap;

use super::{
    augmentation_error::AugmentationError, console_fetcher::ConsoleFetcher,
    template_extractation::TemplateExtractation, AugementKey, AugmentValue, AugmentationResult,
};

pub struct AugementRepository<CF> {
    console_map: HashMap<AugementKey, AugmentValue>,
    console_fetcher: CF,
}

impl<CF: ConsoleFetcher> AugementRepository<CF> {
    pub fn new(console_fetcher: CF) -> Self {
        Self {
            console_map: Default::default(),
            console_fetcher,
        }
    }

    pub fn augment<'a>(&'a mut self, extract: &'a TemplateExtractation) -> AugmentationResult<'a> {
        let value = match extract {
            TemplateExtractation::FromConsole { key, default_value } => {
                if self.console_map.get(*key).is_none() {
                    let may_new_value = self.console_fetcher.fetch_from(key)?;
                    let new_value = match (may_new_value, default_value) {
                        (Some(new_value), _) => new_value,
                        (None, Some(default_value)) => return Ok(default_value),
                        _ => {
                            return Err(AugmentationError::NoValueAndDefaultConsole(
                                key.to_string(),
                            ))
                        }
                    };
                    self.console_map.insert(key.to_string(), new_value);
                }
                self.console_map.get(*key).unwrap()
            }
        };

        Ok(value)
    }
}

#[cfg(test)]
mod testing {
    use crate::template_augmentation::console_fetcher::TestConsoleFetcher;

    use super::*;

    #[test]
    fn get_values_only_once() {
        let expected_value = "World".to_string();
        let key = "WW".to_string();
        let map = hash_map! {
           key.clone() => Some(expected_value.clone()),
        };
        let test_fetcher = TestConsoleFetcher::new(map);

        let mut respo = AugementRepository::new(test_fetcher);

        let extraction = TemplateExtractation::FromConsole {
            key: &key,
            default_value: None,
        };
        let actual = respo.augment(&extraction).unwrap();

        assert_eq!(
            &expected_value,
            actual,
            "Should have returned the value {} to the key {}",
            key.clone(),
            expected_value.clone(),
        );

        let extraction = TemplateExtractation::FromConsole {
            key: &key,
            default_value: Some("Default"),
        };
        let actual_cached = respo.augment(&extraction).unwrap();
        assert_eq!(
            &expected_value, actual_cached,
            "Did not reuse cached value: {}",
            actual_cached
        );
    }

    #[test]
    fn default_no_key_found() {
        let does_not_matter = String::new();
        let map = hash_map! {
           "WW".to_string() => Some(does_not_matter.clone()),
           "YYY".to_string() => Some(does_not_matter)
        };

        let test_fetcher = TestConsoleFetcher::new(map);

        let mut respo = AugementRepository::new(test_fetcher);
        let expected_default = "Default";
        let actual_default = respo
            .augment(&TemplateExtractation::FromConsole {
                key: "xxx",
                default_value: Some("Default"),
            })
            .unwrap();
        assert_eq!(
            expected_default, actual_default,
            "Did not use default value: {}",
            expected_default
        );
    }
}
