use std::io;

use crust_boot_you::{
    app_traits::{file_manipulator::DevOsFileManipulator, path_resolver::DevPathResolver},
    prelude::*,
    DevPathProvider,
};
use tempfile::TempDir;

use crate::common::dir_asserts::DirAssert;

use super::dir_asserts::assert_folders;

mod test_setup_builder;
pub use test_setup_builder::TestSetupBuilder;
#[derive(Debug)]
enum TempedFolder {
    Unnamed(TempDir),
    Named(PathBuf),
}

impl TryFrom<Option<String>> for TempedFolder {
    type Error = io::Error;
    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        if let Some(name) = value {
            let path = std::env::temp_dir().join(name);
            if path.exists() {
                std::fs::remove_dir_all(&path)?;
            }
            std::fs::create_dir_all(&path)?;
            println!("Created named temporary folder at:\n{:?}", &path);
            Ok(TempedFolder::Named(path))
        } else {
            Ok(TempedFolder::Unnamed(TempDir::new().unwrap()))
        }
    }
}

impl TempedFolder {
    pub fn path(&self) -> &Path {
        match self {
            Self::Unnamed(dir) => dir.path(),
            Self::Named(path) => path.as_path(),
        }
    }
}
pub struct TestSetup {
    temp_file: TempedFolder,
    actual: PathBuf,
    expected: Option<PathBuf>,
    os_mani: DevOsFileManipulator,
    path_resolver: DevPathResolver,
    path_provider: DevPathProvider,
}

impl TestSetup {
    pub fn new(actual_expected: (PathBuf, PathBuf)) -> Self {
        let (actual, expected) = actual_expected;
        Self::init_create(actual, Some(expected), None)
    }

    pub fn only_actual(actual: PathBuf) -> Self {
        Self::init_create(actual, None, None)
    }

    fn init_create(actual: PathBuf, expected: Option<PathBuf>, name: Option<String>) -> Self {
        let temp_dir = TempedFolder::try_from(name).unwrap();

        let path_to_temp = temp_dir.path().to_path_buf();
        let os_mani = DevOsFileManipulator::new(&path_to_temp).init_system(actual.to_path_buf());
        os_mani.init_copy_to(&path_to_temp).unwrap_or_else(|error| {
            panic!(
                "Copy actual start content from {:?} to {:?} failed.\nError: {}",
                actual, &path_to_temp, error,
            )
        });
        let temp_path = temp_dir.path();
        let (dev_resolver, dev_provider) = (
            DevPathResolver::new(temp_path.to_path_buf()),
            DevPathProvider::new(temp_path.to_path_buf()),
        );
        Self {
            os_mani,
            temp_file: temp_dir,
            actual,
            path_resolver: dev_resolver,
            path_provider: dev_provider,
            expected,
        }
    }

    pub fn assert_with_expected(&self) {
        let result = match self.expected.as_ref() {
            None => assert_folders(self.path_to_temp(), &self.actual).unwrap(),
            Some(expected_path) => assert_folders(self.path_to_temp(), &expected_path).unwrap(),
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
