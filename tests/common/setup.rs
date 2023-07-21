use crust_boot_you::{app_traits::file_manipulator::DevOsFileManipulator, prelude::*};
use tempfile::TempDir;

use crate::common::dir_asserts::DirAssert;

use super::dir_asserts::assert_folders;
pub struct TestSetup {
    temp_file: TempDir,
    path_to_temp: PathBuf,
    actual: PathBuf,
    expected: PathBuf,
    os_mani: DevOsFileManipulator,
}

impl TestSetup {
    pub fn new(actual_expected: (PathBuf, PathBuf)) -> Self {
        let (actual, expected) = actual_expected;
        dbg!(&actual);
        dbg!(&expected);
        let temp_file = TempDir::new().unwrap();
        let path_to_temp = temp_file.path().to_path_buf();
        let os_mani = DevOsFileManipulator::new(&path_to_temp).init_system(actual.to_path_buf());
        // Use the raw real file manipulator to copy outside the temp folder
        // Actual is outside the temp folder.
        os_mani.init_copy_to(&path_to_temp).unwrap_or_else(|error| {
            panic!(
                "Copieng actual start content from {:?} to {:?} failed.\nError: {}",
                actual, &path_to_temp, error,
            )
        });
        Self {
            // From this point on there is no reason to anything outside the temp folder
            // this file manipulator will prevent any access/write outside temp folder
            os_mani,
            temp_file,
            path_to_temp,
            actual,
            expected,
        }
    }

    pub fn assert_with_expected(&self) {
        let result = assert_folders(&self.path_to_temp, &self.expected).unwrap();
        assert_eq!(DirAssert::Equal, result, "Actual result: {:?}", result);
    }

    pub fn path_to_temp(&self) -> &Path {
        self.path_to_temp.as_path()
    }

    pub fn os_mani(&self) -> &DevOsFileManipulator {
        &self.os_mani
    }
}
