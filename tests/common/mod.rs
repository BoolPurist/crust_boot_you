#![allow(dead_code)]
use crust_boot_you::prelude::*;

use once_cell::sync::Lazy;

pub mod dir_asserts;
pub use setup::TestSetup;
mod setup;

pub use ::function_name::named;

#[macro_export]
macro_rules! actual_expected {
    // `()` indicates that the macro takes no argument.
    () => {{
        let file_name = std::path::Path::new(file!()).file_stem().unwrap();
        let function_name = function_name!();
        let passing = std::path::Path::new(file_name).join(function_name);
        crate::common::get_actual_expected_diff_dir_assert(&passing)
    }};
}

const DATA_CASES: &str = "data_cases";
const ACTUAL: &str = "actual";
const EXPECTED: &str = "expected";
const APP_TEST_CASES: &str = "app_test_cases";
const FOR_TEST_DIR_ASSERT: &str = "for_test_dir_assert";
const TEST_DATA_SETUPS: &str = "test_data_setups";

static DATA_TEST_ROOT: Lazy<PathBuf> =
    Lazy::new(|| Path::new(constants::project_root()).join(TEST_DATA_SETUPS));
static DATA_APP_TEST_ROOT: Lazy<PathBuf> = Lazy::new(|| DATA_TEST_ROOT.join(APP_TEST_CASES));
static DATA_TEST_DIR_ASSERT: Lazy<PathBuf> = Lazy::new(|| DATA_TEST_ROOT.join(FOR_TEST_DIR_ASSERT));

pub fn get_actual_expected_diff_dir_assert(name: &Path) -> (PathBuf, PathBuf) {
    let (actual, exepect) = (
        DATA_TEST_ROOT.join(name).join(ACTUAL),
        DATA_TEST_ROOT.join(name).join(EXPECTED),
    );
    return (
        actual.canonicalize().unwrap_or_else(|_| {
            panic!(
                "Actual folder does not exist at\n{:?}\n{}",
                actual,
                hint_err_msg(name, ACTUAL)
            )
        }),
        exepect.canonicalize().unwrap_or_else(|_| {
            panic!(
                "Expected folder does not exist at\n{:?}\n{}",
                exepect,
                hint_err_msg(name, EXPECTED)
            )
        }),
    );

    fn hint_err_msg(name: &Path, input_name: &str) -> String {
        let help = name
            .to_string_lossy()
            .split(std::path::MAIN_SEPARATOR_STR)
            .chain(std::iter::once(input_name))
            .map(|to_string| to_string.to_string())
            .collect::<Vec<String>>()
            .join(" -> ");
        format!(
            "Create the follwing folders {} under {:?}",
            help, *DATA_TEST_ROOT
        )
    }
}