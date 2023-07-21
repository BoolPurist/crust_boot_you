use crate::prelude::*;
use std::path::PathBuf;

use super::PathProvider;

#[derive(Debug)]
pub struct TestPathProvider {
    data: PathBuf,
    config: PathBuf,
    cwd: PathBuf,
}

impl TestPathProvider {
    pub fn new(data: PathBuf, config: PathBuf, cwd: PathBuf) -> Self {
        Self { data, config, cwd }
    }

    pub fn clone_from(
        data: impl AsRef<Path>,
        config: impl AsRef<Path>,
        cwd: impl AsRef<Path>,
    ) -> Self {
        let (data, config, cwd): (PathBuf, PathBuf, PathBuf) = (
            data.as_ref().to_path_buf(),
            config.as_ref().to_path_buf(),
            cwd.as_ref().to_path_buf(),
        );
        Self::new(data, config, cwd)
    }
}

impl PathProvider for TestPathProvider {
    fn data(&self) -> PathResult {
        Ok(self.data.clone())
    }

    fn config(&self) -> PathResult {
        Ok(self.config.clone())
    }

    fn cwd(&self) -> PathResult {
        Ok(self.cwd.clone())
    }
}
