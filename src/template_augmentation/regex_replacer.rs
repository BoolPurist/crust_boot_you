use crate::cli::LoadTemplateArg;
use crate::{prelude::*, ValidPlaceholderBorder};
use std::borrow::Cow;

use regex::Regex;

use super::console_fetcher::TestConsoleFetcher;
use super::TestAugmentStore;

use super::template_extractation::ExtractForConsole;
use super::{
    augmentation_error::AugmentationError,
    console_fetcher::{ConsoleFetcher, IoConsoleFetcher},
    template_extractation::TemplateExtractation,
    AugementRepository, TemplateAugmentor,
};

fn build_placehold_matcher(left: &str, right: &str) -> Regex {
    let escpaded_left = regex::escape(left);
    let escpaded_right = regex::escape(right);

    // Example: with left: %%
    // Example: with right: %%
    // Matches things like "%%aaa%%"
    let regex = format!(
        "{}(?<value>[^{}]*){}",
        escpaded_left, escpaded_right, escpaded_right
    );
    Regex::new(&regex).expect("Incorrect regex matcher for a key to be replaced within a template")
}

pub struct RegexTemplateAugmentor<CF> {
    cache: AugementRepository<CF>,
    v_default_sep: ValidPlaceholderBorder,
    placeholder_matcher: Regex,
}

impl RegexTemplateAugmentor<IoConsoleFetcher> {
    pub fn prod_new(agrs: &LoadTemplateArg) -> Self {
        let console_fetcher = IoConsoleFetcher;
        let cache = AugementRepository::new(console_fetcher);
        Self::from_cli(cache, agrs)
    }
}

impl<CF> RegexTemplateAugmentor<CF>
where
    CF: ConsoleFetcher,
{
    pub fn new(cache: AugementRepository<CF>) -> Self {
        Self {
            cache,
            v_default_sep: ValidPlaceholderBorder::new(
                constants::SEPERATOR_BETWEEN_DEFAULT_AND_VALUE.to_string(),
            )
            .unwrap(),
            placeholder_matcher: build_placehold_matcher(
                constants::DEFAULT_LEFT_DELIMITER,
                constants::DEFAULT_RIGHT_DELIMITER,
            ),
        }
    }

    pub fn from_cli(cache: AugementRepository<CF>, args: &LoadTemplateArg) -> Self {
        let details = args.details();
        let (left, right) = (details.left_delimiter(), details.right_delimiter());
        Self {
            cache,
            v_default_sep: details.sep_val_default().clone(),
            placeholder_matcher: build_placehold_matcher(left, right),
        }
    }
    pub fn direct_new(cache: AugementRepository<CF>, left: &str, right: &str) -> Self {
        Self {
            cache,
            v_default_sep: ValidPlaceholderBorder::new(
                constants::SEPERATOR_BETWEEN_DEFAULT_AND_VALUE.to_string(),
            )
            .unwrap(),
            placeholder_matcher: build_placehold_matcher(left, right),
        }
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
        let mut captures = self.placeholder_matcher.captures_iter(input).peekable();
        let needs_expansion = captures.peek().is_some();
        if needs_expansion {
            let mut expanded = String::with_capacity(input.len());
            let mut last_match = 0;
            for found in captures {
                let matched_range = found.get(0).unwrap();
                expanded.push_str(&input[last_match..matched_range.start()]);

                let extraction = {
                    let value = &found["value"];
                    let mut splited = value.split(self.v_default_sep.as_str());
                    let (key, default_value) = (splited.next().unwrap(), splited.next());
                    TemplateExtractation::FromConsole(ExtractForConsole::new(key, default_value))
                };
                let replacement = self
                    .cache
                    .augment(&extraction)
                    .map_err(|error| AugmentationError::new(input, matched_range.start(), error))?;
                expanded.push_str(replacement);
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
    use std::collections::HashMap;

    use crate::template_augmentation::console_fetcher::TestConsoleFetcher;

    use super::*;

    #[test]
    fn regex_extract_one_value() {
        let input = "aa aaa {{value to get}} aaa";
        let actual = build_placehold_matcher("{{", "}}").captures(input).unwrap();
        let actual_extracted = &actual["value"];
        assert_eq!("value to get", actual_extracted);
    }

    #[test]
    fn regex_extract_do_not_change_after_error() {
        let input = "aa aaa {{value to get}} {{!!}} {{value to get}} ";
        let map = hash_map! {
            "value to get".to_string() => "YYY".to_string(),
            "xxx".to_string() => "XXX".to_string(),
        };

        let _ = assert_standard_left_right_delimiters(input, map).unwrap_err();
    }

    #[test]
    fn regex_extract_two_value() {
        let input = "aa aaa {{value to get}} {{xxx}}  ";
        let map = hash_map! {
            "value to get".to_string() => "YYY".to_string(),
            "xxx".to_string() => "XXX".to_string(),
        };

        let actual = assert_standard_left_right_delimiters(input, map).unwrap();

        assert_eq!("aa aaa YYY XXX  ", actual);
    }

    #[test]
    fn regex_extract_one_and_other_default_values() {
        let input = "aa aaa {{value to get}} {{!!?A}} {{!!?B}} ";
        let map = hash_map! {
            "value to get".to_string() => "YYY".to_string(),
            "xxx".to_string() => "XXX".to_string(),
        };

        let actual = assert_standard_left_right_delimiters(input, map).unwrap();

        assert_eq!("aa aaa YYY A B ", actual);
    }

    #[test]
    fn regex_extract_and_no_default() {
        let input = "aa aaa {{value to get}} {{!!?A}} {{!!?B}} ";
        let map = hash_map! {
            "value to get".to_string() => "YYY".to_string(),
            "!!".to_string() => "XXX".to_string(),
        };

        let actual = assert_standard_left_right_delimiters(input, map).unwrap();
        assert_eq!("aa aaa YYY XXX XXX ", actual);
    }

    #[test]
    fn regex_double_precent() {
        let input = r#"aa aaa {{value to get}} {{!!?A}} {{!!?B}} %%AAA%% 
                   %%world%%  {{world}} %%world%%
                      %%a?b%%
        "#;
        let map = hash_map! {
            "world".to_string() => "P x!!x C".to_string(),
            "value to get".to_string() => "YYY".to_string(),
            "!!".to_string() => "XXX".to_string(),
            "AAA".to_string() => "BBB".to_string(),
        };

        let actual = assert_custom_right_left_matcher(input, map, ("%%", "%%"));
        insta::assert_display_snapshot!(actual);
    }

    #[test]
    fn regex_double_inter_precent() {
        let input = r#"
                    {{world}} %%world%%
                     %%a?b%%"#;
        let map = hash_map! {
            "world".to_string() => "P x!!x C".to_string(),
            "value to get".to_string() => "YYY".to_string(),
            "!!".to_string() => "XXX".to_string(),
            "AAA".to_string() => "BBB".to_string(),
        };

        let actual = assert_custom_right_left_matcher(input, map, ("%%", "%%"));
        insta::assert_display_snapshot!(actual);
    }

    #[test]
    fn regex_arrow_a_b_precent_r_arrow() {
        let input = r#"
                    {{world}} <ABworld%>
                     %%a?b%%"#;
        let map = hash_map! {
            "world".to_string() => "P x!!x C".to_string(),
            "value to get".to_string() => "YYY".to_string(),
            "!!".to_string() => "XXX".to_string(),
            "AAA".to_string() => "BBB".to_string(),
        };

        let actual = assert_custom_right_left_matcher(input, map, ("<AB", "%>"));
        insta::assert_display_snapshot!(actual);
    }

    fn assert_custom_right_left_matcher(
        input: &str,
        map: HashMap<String, String>,
        (left, right): (&str, &str),
    ) -> String {
        let test_augmenter = TestConsoleFetcher::new(map);
        let respo = AugementRepository::new(test_augmenter);
        let mut regex_augmentor = RegexTemplateAugmentor::direct_new(respo, left, right);
        let actual = regex_augmentor.try_replace(&input).unwrap();

        actual.to_string()
    }

    fn assert_standard_left_right_delimiters(
        input: &str,
        map: HashMap<String, String>,
    ) -> Result<Cow<'_, str>, AugmentationError> {
        let test_augmenter = TestConsoleFetcher::new(map);
        let respo = AugementRepository::new(test_augmenter);
        let mut regex_augmentor = RegexTemplateAugmentor::new(respo);
        regex_augmentor.try_replace(&input)
    }
}
