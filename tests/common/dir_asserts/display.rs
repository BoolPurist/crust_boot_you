use colored::*;
use colored::{Color, ColoredString};
use itertools::Itertools;

use crate::common::{prelude::*, ACTUAL, EXPECTED};
use std::{fmt::Display, fs::FileType};

use super::DirAssert;

impl Display for DirAssert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: String = match self {
            Self::Equal => "Both folders are equal".to_owned(),
            Self::DifferentPaths((left, right)) => diff_paths(left, right),
            Self::DifferentFileType((path, _is_same), (actual, expected)) => {
                diff_types(path, actual, expected)
            }
            Self::DifferentContent((path, _is_same), (actual, expected)) => {
                diff_contents(path, &actual, &expected)
            }
            Self::ActualMore(left) => actual_more(left),
            Self::ExpectedMore(rigt) => expected_more(rigt),
        };

        return write!(f, "{}", output);

        fn diff_paths(actual: &Path, expected: &Path) -> String {
            format!(
                "{}\n{}\n\n{}",
                title(),
                label_color("Different paths"),
                diff_paths_line(actual, expected),
            )
        }

        fn diff_types(same: &Path, actual: &FileType, expected: &FileType) -> String {
            let (actual, expected) = (
                (filte_type_to_txt(actual).color(actual_color())),
                filte_type_to_txt(expected).color(expected_color()),
            );
            format!(
                "{}\n{}\n{}\n{}: {}\n{}: {}",
                title(),
                label_color("Different types"),
                path_line(same),
                actual_txt(),
                actual,
                expected_txt(),
                expected
            )
        }

        fn diff_contents(same: &Path, actual: &str, expected: &str) -> String {
            [
                title(),
                label_color("Differnet content").to_string(),
                path_line(same).to_string(),
                "\n===========================\n".to_string(),
                format!(
                    "{} | {}",
                    "Missing in exepcted".red(),
                    "Missing in actual".green()
                ),
                "".to_string(),
                different_from_text(actual, expected),
            ]
            .join("\n")
        }

        fn actual_more(actual: &Path) -> String {
            let actual = to_actual_path_txt(actual);
            format!(
                "{}\n{}: {}",
                label_color("Found a file which is not in expected."),
                actual_txt(),
                actual
            )
        }

        fn expected_more(expected: &Path) -> String {
            let expected = to_expected_path_txt(expected);
            format!(
                "{}.\n{}: {}",
                label_color("A file is missing in actual").red(),
                expected_txt(),
                expected
            )
        }

        fn diff_paths_line(left: &Path, right: &Path) -> String {
            format!(
                "{0}: {2:?}\n{1}: {3:?}",
                actual_txt(),
                expected_txt(),
                left,
                right
            )
        }

        fn title() -> String {
            format!("{0} != {1}", actual_txt(), expected_txt(),)
        }

        fn path_line(path: &Path) -> ColoredString {
            format!("Path: ({:?})", path).italic()
        }

        fn filte_type_to_txt(file_type: &FileType) -> &'static str {
            if file_type.is_file() {
                "file"
            } else if file_type.is_dir() {
                "dir"
            } else if file_type.is_symlink() {
                "symlink"
            } else {
                unreachable!()
            }
        }

        fn to_actual_path_txt(actual: &Path) -> ColoredString {
            actual.to_string_lossy().color(actual_color())
        }

        fn to_expected_path_txt(expected: &Path) -> ColoredString {
            expected.to_string_lossy().color(expected_color())
        }

        fn actual_txt() -> ColoredString {
            ACTUAL.color(actual_color())
        }

        fn expected_txt() -> ColoredString {
            EXPECTED.color(expected_color())
        }

        fn label_color(message: &str) -> ColoredString {
            message.color(Color::Red)
        }

        fn actual_color() -> Color {
            Color::Yellow
        }
        fn expected_color() -> Color {
            Color::Blue
        }

        fn different_from_text(actual: &str, expected: &str) -> String {
            use difference::{Changeset, Difference};
            // Compare both texts, the third parameter defines the split level.
            let Changeset { diffs, .. } = Changeset::new(actual, expected, "\n");

            diffs
                .into_iter()
                .map(|next_diff| {
                    let color = match next_diff {
                        Difference::Same(ref x) => x.white().to_string(),
                        Difference::Add(ref x) => x.green().to_string(),
                        Difference::Rem(ref x) => x.red().to_string(),
                    };
                    format!(": {}", color)
                })
                .join("\n")
        }
    }
}
