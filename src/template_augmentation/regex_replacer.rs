use crate::prelude::*;
use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::{Regex, Replacer};

use super::{
    augmentation_error::AugmentationError, console_fetcher::ConsoleFetcher,
    template_extractation::TemplateExtractation, AugementRepository, TemplateAugmentor,
};

fn build_regex_template() -> Regex {
    Regex::new(r#"\{\{(?<value>[^\{]*)\}\}"#).unwrap()
}

static REGEX_TEMPLATE: Lazy<Regex> = Lazy::new(build_regex_template);

pub struct RegexTemplateAugmentor<CF> {
    cache: AugementRepository<CF>,
}

impl<CF> RegexTemplateAugmentor<CF> {
    pub fn new(cache: AugementRepository<CF>) -> Self {
        Self { cache }
    }
}

impl<CF> TemplateAugmentor for RegexTemplateAugmentor<CF>
where
    CF: ConsoleFetcher,
{
    fn try_replace<'a>(&mut self, input: &'a str) -> Result<Cow<'a, str>, AugmentationError> {
        let mut found_error = None;
        let output = {
            let replacer = RegexReplacer::new(&mut self.cache, &mut found_error);
            let output = REGEX_TEMPLATE.replace_all(input, replacer);
            output
        };

        match found_error {
            Some(error) => Err(error.clone()),
            None => Ok(output),
        }
    }
}

pub struct RegexReplacer<'a, CF> {
    respo_store: &'a mut AugementRepository<CF>,
    found_error: &'a mut Option<AugmentationError>,
}

impl<'a, CF> RegexReplacer<'a, CF> {
    pub fn new(
        respo_store: &'a mut AugementRepository<CF>,
        found_error: &'a mut Option<AugmentationError>,
    ) -> Self {
        Self {
            respo_store,
            found_error,
        }
    }
}

impl<'a, CF: ConsoleFetcher> Replacer for RegexReplacer<'a, CF> {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let extraction = {
            let value = &caps["value"];
            let mut splited = value.split(constants::SEPERATOR_BETWEEN_DEFAULT_AND_VALUE);
            let (key, default_value) = (splited.next().unwrap(), splited.next());
            TemplateExtractation::FromConsole { key, default_value }
        };

        let found_error = &mut self.found_error;
        if found_error.is_none() {
            match self.respo_store.augment(&extraction) {
                Ok(value) => dst.push_str(value),
                Err(error) => {
                    insert_original(caps, dst);
                    **found_error = Some(error)
                }
            }
        } else {
            insert_original(caps, dst);
        }

        fn insert_original(caps: &regex::Captures<'_>, dst: &mut String) {
            let original = caps.get(0).unwrap();
            dst.push_str(original.as_str());
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::template_augmentation::console_fetcher::TestConsoleFetcher;

    use super::*;

    #[test]
    fn regex_extract_one_value() {
        let input = "aa aaa {{value to get}} aaa";
        let actual = REGEX_TEMPLATE.captures(input).unwrap();
        let actual_extracted = &actual["value"];
        assert_eq!("value to get", actual_extracted);
    }
    #[test]
    fn regex_extract_two_value() {
        let input = "aa aaa {{value to get}} {{xxx}}  ";
        let map = hash_map! {
            "value to get".to_string() => Some("YYY".to_string()),
            "xxx".to_string() => Some("XXX".to_string()),
        };
        let test_augmenter = TestConsoleFetcher::new(map);

        let respo = AugementRepository::new(test_augmenter);
        let mut regex_augmentor = RegexTemplateAugmentor::new(respo);
        let actual = regex_augmentor.try_replace(&input).unwrap();

        assert_eq!("aa aaa YYY XXX  ", actual);
    }
    #[test]
    fn regex_extract_do_not_change_after_error() {
        let input = "aa aaa {{value to get}} {{!!}} {{value to get}} ";
        let map = hash_map! {
            "value to get".to_string() => Some("YYY".to_string()),
            "xxx".to_string() => Some("XXX".to_string()),
        };
        let test_augmenter = TestConsoleFetcher::new(map);

        let respo = AugementRepository::new(test_augmenter);
        let mut regex_augmentor = RegexTemplateAugmentor::new(respo);
        let actual = regex_augmentor.try_replace(&input).unwrap_err();

        assert_eq!(
            AugmentationError::NoValueAndDefaultConsole(String::from("!!")),
            actual,
        );
    }
    #[test]
    fn regex_extract_one_and_other_default_values() {
        let input = "aa aaa {{value to get}} {{!!?A}} {{!!?B}} ";
        let map = hash_map! {
            "value to get".to_string() => Some("YYY".to_string()),
            "xxx".to_string() => Some("XXX".to_string()),
        };
        let test_augmenter = TestConsoleFetcher::new(map);

        let respo = AugementRepository::new(test_augmenter);
        let mut regex_augmentor = RegexTemplateAugmentor::new(respo);
        let actual = regex_augmentor.try_replace(&input).unwrap();

        assert_eq!("aa aaa YYY A B ", actual);
    }

    #[test]
    fn regex_extract_and_no_default() {
        let input = "aa aaa {{value to get}} {{!!?A}} {{!!?B}} ";
        let map = hash_map! {
            "value to get".to_string() => Some("YYY".to_string()),
            "!!".to_string() => Some("XXX".to_string()),
        };
        let test_augmenter = TestConsoleFetcher::new(map);

        let respo = AugementRepository::new(test_augmenter);
        let mut regex_augmentor = RegexTemplateAugmentor::new(respo);
        let actual = regex_augmentor.try_replace(&input).unwrap();

        assert_eq!("aa aaa YYY XXX XXX ", actual);
    }
}
