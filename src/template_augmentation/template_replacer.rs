#![allow(dead_code)]
use crate::prelude::*;
/// TODO: dead_code
use std::{borrow::Cow, cell::RefCell, rc::Rc};

use once_cell::sync::Lazy;
use regex::{Regex, Replacer};

use super::{
    augmentation_error::AugmentationError, console_fetcher::ConsoleFetcher,
    template_extractation::TemplateExtractation, AugementRepository,
};

fn build_regex_template() -> Regex {
    Regex::new(r#"\{\{(?<value>[^\{]*)\}\}"#).unwrap()
}

static REGEX_TMPLATE: Lazy<Regex> = Lazy::new(build_regex_template);

pub struct TemplateReplacer<'a, CF> {
    respo_store: &'a mut AugementRepository<CF>,
    found_error: Rc<RefCell<Option<AugmentationError>>>,
}

impl<'a, CF: ConsoleFetcher> TemplateReplacer<'a, CF> {
    pub fn try_replace<'s>(
        regex: &Regex,
        input: &'s str,
        respo_store: &'a mut AugementRepository<CF>,
    ) -> Result<Cow<'s, str>, AugmentationError> {
        let found_error = Rc::new(RefCell::new(None));
        let output = {
            let self_r = Self {
                respo_store,
                found_error: Rc::clone(&found_error),
            };
            let output = regex.replace_all(input, self_r);
            output
        };

        let may_error = found_error.borrow();
        match &*may_error {
            Some(error) => Err(error.clone()),
            None => Ok(output),
        }
    }
}

impl<'a, CF: ConsoleFetcher> Replacer for TemplateReplacer<'a, CF> {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let extraction = {
            let value = &caps["value"];
            let mut splited = value.split(constants::SEPERATOR_BETWEEN_DEFAULT_AND_VALUE);
            let (key, default_value) = (splited.next().unwrap(), splited.next());
            TemplateExtractation::FromConsole { key, default_value }
        };

        let mut may_error = self.found_error.borrow_mut();
        if may_error.is_none() {
            match self.respo_store.augment(&extraction) {
                Ok(value) => dst.push_str(value),
                Err(error) => {
                    insert_original(caps, dst);
                    *may_error = Some(error)
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
        let actual = REGEX_TMPLATE.captures(input).unwrap();
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

        let mut respo = AugementRepository::new(test_augmenter);
        let actual = TemplateReplacer::try_replace(&REGEX_TMPLATE, &input, &mut respo).unwrap();

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

        let mut respo = AugementRepository::new(test_augmenter);
        let replacer =
            TemplateReplacer::try_replace(&REGEX_TMPLATE, &input, &mut respo).unwrap_err();

        assert_eq!(
            AugmentationError::NoValueAndDefaultConsole(String::from("!!")),
            replacer,
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

        let mut respo = AugementRepository::new(test_augmenter);
        let actual = TemplateReplacer::try_replace(&REGEX_TMPLATE, &input, &mut respo).unwrap();

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

        let mut respo = AugementRepository::new(test_augmenter);
        let actual = TemplateReplacer::try_replace(&REGEX_TMPLATE, &input, &mut respo).unwrap();

        assert_eq!("aa aaa YYY XXX XXX ", actual);
    }
}
