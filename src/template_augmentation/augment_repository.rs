use std::collections::HashMap;

use super::{
    console_fetcher::ConsoleFetcher,
    template_extractation::{ExtractForConsole, TemplateExtractation},
    AugmentationResult,
};

/// We do not need to clone
type RespositoryKey = Box<str>;
type ResolvedValue = Option<RespositoryKey>;

pub struct AugementRepository<CF> {
    console_map: HashMap<RespositoryKey, ResolvedValue>,
    console_fetcher: CF,
}

impl<CF: ConsoleFetcher> AugementRepository<CF> {
    pub fn new(console_fetcher: CF) -> Self {
        Self {
            console_map: Default::default(),
            console_fetcher,
        }
    }

    pub fn augment<'a>(
        &'a mut self,
        extract: &'a TemplateExtractation<'a>,
    ) -> AugmentationResult<'a> {
        match extract {
            TemplateExtractation::FromConsole(extract) => {
                let ExtractForConsole { key, default_value } = extract;
                if !self.console_map.contains_key(*key) {
                    let may_new_value: ResolvedValue = self
                        .console_fetcher
                        .fetch_from(extract)?
                        .map(|key| key.as_str().into());

                    debug_assert!(
                        may_new_value
                            .as_ref()
                            .map(|no_trailing_newline| !no_trailing_newline.ends_with('\n'))
                            .unwrap_or(true),
                        "Read value should not have a new line at the end"
                    );

                    let first_time = self
                        .console_map
                        .insert((*key).into(), may_new_value)
                        .is_none();
                    debug_assert!(first_time);
                }

                let fetched_key = self.console_map.get(*key);
                debug_assert!(
                    fetched_key.is_some(),
                    "Key {} should have been already inserted",
                    key
                );
                match (fetched_key, default_value) {
                    (Some(Some(resolved)), _) => Ok(resolved),
                    (Some(None), Some(default)) => {
                        info!("Using default value {} for key {}", default, key);
                        Ok(default)
                    }
                    _ => Err(anyhow!(
                        "Key ({}): Could not be retrieved from console and has no default value ",
                        key.to_string()
                    )),
                }
            }
        }
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
           key.clone() => expected_value.clone(),
        };
        let test_fetcher = TestConsoleFetcher::new(map);

        let mut respo = AugementRepository::new(test_fetcher);

        let extraction = TemplateExtractation::FromConsole(ExtractForConsole::new(&key, None));
        let actual = respo.augment(&extraction).unwrap();

        assert_eq!(
            expected_value.as_str(),
            actual,
            "Should have returned the value {} to the key {}",
            key.clone(),
            expected_value.clone(),
        );

        let extraction =
            TemplateExtractation::FromConsole(ExtractForConsole::new(&key, Some("Default")));
        let actual_cached = respo.augment(&extraction).unwrap();
        assert_eq!(
            expected_value.as_str(),
            actual_cached,
            "Did not reuse cached value: {}",
            actual_cached
        );
    }

    #[test]
    fn default_no_key_found() {
        let does_not_matter = String::new();
        let map = hash_map! {
           "WW".to_string() => does_not_matter.clone(),
           "YYY".to_string() => does_not_matter
        };

        let test_fetcher = TestConsoleFetcher::new(map);

        let mut respo = AugementRepository::new(test_fetcher);
        let expected_default = "Default";
        let extract =
            TemplateExtractation::FromConsole(ExtractForConsole::new("xxx", Some("Default")));
        let actual_default = respo.augment(&extract).unwrap();
        assert_eq!(
            expected_default, actual_default,
            "Did not use default value: {}",
            expected_default
        );
    }
}
