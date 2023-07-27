use crate::prelude::*;
use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::Regex;

use super::console_fetcher::TestConsoleFetcher;
use super::TestAugmentStore;

use super::{
    augmentation_error::AugmentationError,
    console_fetcher::{ConsoleFetcher, IoConsoleFetcher},
    template_extractation::TemplateExtractation,
    AugementRepository, TemplateAugmentor,
};

fn build_regex_template() -> Regex {
    Regex::new(r#"\{\{(?<value>[^\{]*)\}\}"#)
        .expect("Incorrect regex matcher for a key to be replaced within a template")
}

static REGEX_TEMPLATE: Lazy<Regex> = Lazy::new(build_regex_template);

pub struct RegexTemplateAugmentor<CF> {
    cache: AugementRepository<CF>,
}

impl Default for RegexTemplateAugmentor<IoConsoleFetcher> {
    fn default() -> Self {
        let console_fetcher = IoConsoleFetcher;
        let cache = AugementRepository::new(console_fetcher);
        Self::new(cache)
    }
}

impl<CF: ConsoleFetcher> RegexTemplateAugmentor<CF> {
    pub fn new(cache: AugementRepository<CF>) -> Self {
        Self { cache }
    }
}

impl RegexTemplateAugmentor<TestConsoleFetcher> {
    pub fn from_fake(map: TestAugmentStore) -> Self {
        let fake_console_fetcher = TestConsoleFetcher::new(map);
        let store = AugementRepository::new(fake_console_fetcher);
        Self::new(store)
    }
}

impl<CF> TemplateAugmentor for RegexTemplateAugmentor<CF>
where
    CF: ConsoleFetcher,
{
    fn try_replace<'a>(&mut self, input: &'a str) -> Result<Cow<'a, str>, AugmentationError> {
        let mut captures = REGEX_TEMPLATE.captures_iter(input).peekable();
        let needs_expansion = captures.peek().is_some();
        if needs_expansion {
            let mut expanded = String::with_capacity(input.len());
            let mut last_match = 0;
            for found in captures {
                let matched_range = found.get(0).unwrap();
                expanded.push_str(&input[last_match..matched_range.start()]);

                let extraction = {
                    let value = &found["value"];
                    let mut splited = value.split(constants::SEPERATOR_BETWEEN_DEFAULT_AND_VALUE);
                    let (key, default_value) = (splited.next().unwrap(), splited.next());
                    TemplateExtractation::FromConsole { key, default_value }
                };
                let replacement = self
                    .cache
                    .augment(&extraction)
                    .map_err(|error| AugmentationError::new(input, matched_range.start(), error))?;
                expanded.push_str(&replacement);
                last_match = matched_range.end();
            }

            expanded.push_str(&input[last_match..]);
            Ok(Cow::Owned(expanded))
        } else {
            Ok(Cow::Borrowed(input))
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
            "value to get".to_string() => "YYY".to_string(),
            "xxx".to_string() => "XXX".to_string(),
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
            "value to get".to_string() => "YYY".to_string(),
            "xxx".to_string() => "XXX".to_string(),
        };
        let test_augmenter = TestConsoleFetcher::new(map);

        let respo = AugementRepository::new(test_augmenter);
        let mut regex_augmentor = RegexTemplateAugmentor::new(respo);
        let _ = regex_augmentor.try_replace(&input).unwrap_err();
    }
    #[test]
    fn regex_extract_one_and_other_default_values() {
        let input = "aa aaa {{value to get}} {{!!?A}} {{!!?B}} ";
        let map = hash_map! {
            "value to get".to_string() => "YYY".to_string(),
            "xxx".to_string() => "XXX".to_string(),
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
            "value to get".to_string() => "YYY".to_string(),
            "!!".to_string() => "XXX".to_string(),
        };
        let test_augmenter = TestConsoleFetcher::new(map);

        let respo = AugementRepository::new(test_augmenter);
        let mut regex_augmentor = RegexTemplateAugmentor::new(respo);
        let actual = regex_augmentor.try_replace(&input).unwrap();

        assert_eq!("aa aaa YYY XXX XXX ", actual);
    }
}
