#![allow(dead_code)]
use crust_boot_you::prelude::*;

use once_cell::sync::Lazy;

pub mod dir_asserts;
pub use setup::TestSetup;
mod setup;

pub use ::function_name::named;

#[cfg(test)]
pub mod prelude {
    pub use super::setup::TestSetup;
    pub use ::function_name::named;
    pub use crust_boot_you::app_traits::path_provider::TestPathProvider;
    pub use crust_boot_you::prelude::*;
}
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
#[macro_export]
macro_rules! actual {
    // `()` indicates that the macro takes no argument.
    () => {{
        let file_name = std::path::Path::new(file!()).file_stem().unwrap();
        let function_name = function_name!();
        let passing = std::path::Path::new(file_name).join(function_name);
        crate::common::get_actual_diff_dir_assert(&passing)
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

pub fn get_actual_diff_dir_assert(name: &Path) -> PathBuf {
    get_diff_dir_assert(name, ACTUAL)
}

pub fn get_actual_expected_diff_dir_assert(name: &Path) -> (PathBuf, PathBuf) {
    (
        get_diff_dir_assert(name, ACTUAL),
        get_diff_dir_assert(name, EXPECTED),
    )
}

pub fn strip_away_changing_temp_prefix(
    prefix: &Path,
    to_strip: &[impl AsRef<Path>],
) -> Vec<PathBuf> {
    to_strip
        .iter()
        .map(|to_strip| {
            let new_path = to_strip.as_ref().strip_prefix(prefix).unwrap().to_owned();
            new_path
        })
        .collect()
}

fn get_diff_dir_assert(name: &Path, base: &str) -> PathBuf {
    let given = DATA_TEST_ROOT.join(name).join(base);

    return given.canonicalize().unwrap_or_else(|_| {
        panic!(
            "Actual folder does not exist at\n{:?}\n{}",
            given,
            hint_err_msg(name, base)
        )
    });

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
