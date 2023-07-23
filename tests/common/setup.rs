use crust_boot_you::{
    app_traits::{file_manipulator::DevOsFileManipulator, path_resolver::DevPathResolver},
    prelude::*,
    DevPathProvider,
};
use tempfile::{Builder, TempDir};

use crate::common::dir_asserts::DirAssert;

use super::dir_asserts::assert_folders;

pub struct TestSetup {
    temp_file: TempDir,
    only_actual: bool,
    actual: PathBuf,
    expected: PathBuf,
    os_mani: DevOsFileManipulator,
    path_resolver: DevPathResolver,
    path_provider: DevPathProvider,
}

impl TestSetup {
    pub fn new(actual_expected: (PathBuf, PathBuf)) -> Self {
        let (actual, expected) = actual_expected;
        Self::init_create(None, actual, expected, false)
    }

    pub fn only_actual(actual: PathBuf) -> Self {
        Self::init_create(None, actual.clone(), actual, true)
    }

    fn init_create(
        name: Option<ValidTemplateName>,
        actual: PathBuf,
        expected: PathBuf,
        only_actual: bool,
    ) -> Self {
        let temp_file = if let Some(name) = name {
            Builder::new()
                .suffix("")
                .tempdir_in(std::env::temp_dir().join(name.as_ref()))
                .unwrap()
        } else {
            TempDir::new().unwrap()
        };
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
        let temp_path = temp_file.path();
        let (dev_resolver, dev_provider) = (
            DevPathResolver::new(temp_path.to_path_buf()),
            DevPathProvider::new(temp_path.to_path_buf()),
        );
        Self {
            // From this point on there is no reason to anything outside the temp folder
            // this file manipulator will prevent any access/write outside temp folder
            os_mani,
            temp_file,
            only_actual,
            path_resolver: dev_resolver,
            path_provider: dev_provider,
            actual,
            expected,
        }
    }

    pub fn assert_with_expected(&self) {
        let result = if self.only_actual {
            assert_folders(self.path_to_temp(), &self.actual).unwrap()
        } else {
            assert_folders(self.path_to_temp(), &self.expected).unwrap()
        };
        assert!(DirAssert::Equal == result, "{}", result);
    }

    pub fn path_to_temp(&self) -> &Path {
        self.temp_file.path()
    }

    pub fn os_mani(&self) -> &DevOsFileManipulator {
        &self.os_mani
    }

    pub fn path_resolver(&self) -> &DevPathResolver {
        &self.path_resolver
    }

    pub fn path_provider(&self) -> &DevPathProvider {
        &self.path_provider
    }
}
