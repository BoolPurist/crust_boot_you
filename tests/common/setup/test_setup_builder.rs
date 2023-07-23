use crust_boot_you::prelude::*;

use super::TestSetup;
pub struct TestSetupBuilder {
    suffix_cwd: Option<PathBuf>,
    actual: PathBuf,
    expected: Option<PathBuf>,
}

impl TestSetupBuilder {
    pub fn new(actual_expected: (PathBuf, PathBuf)) -> Self {
        let (actual, expected) = actual_expected;
        Self {
            actual,
            expected: Some(expected),
            suffix_cwd: None,
        }
    }

    pub fn only_actual(actual: PathBuf) -> Self {
        Self {
            actual,
            expected: None,
            suffix_cwd: None,
        }
    }

    pub fn suffix_cwd(mut self, new_suffix_cwd: PathBuf) -> Self {
        self.suffix_cwd = Some(new_suffix_cwd);
        self
    }

    pub fn build(self) -> TestSetup {
        let mut setup = match self.expected {
            Some(given_expected) => TestSetup::new((self.actual, given_expected)),
            None => TestSetup::only_actual(self.actual.clone()),
        };

        if let Some(cwd) = self.suffix_cwd {
            let new_cwd = setup.os_mani.cwd().unwrap().join(cwd);
            setup.os_mani.set_cwd(new_cwd);
        }

        setup
    }
}
