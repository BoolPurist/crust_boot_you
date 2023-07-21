use crate::prelude::*;
use std::path::PathBuf;

use super::PathProvider;

#[derive(Debug)]
pub struct TestPathProvider {
    root: PathBuf,
    data: PathBuf,
    config: PathBuf,
    cwd: PathBuf,
}

impl TestPathProvider {
    pub fn new(root: PathBuf, data: PathBuf, config: PathBuf, cwd: PathBuf) -> Self {
        Self {
            root,
            data,
            config,
            cwd,
        }
    }

    pub fn clone_from(
        root: impl AsRef<Path>,
        data: impl AsRef<Path>,
        config: impl AsRef<Path>,
        cwd: impl AsRef<Path>,
    ) -> Self {
        let (root, data, config, cwd): (PathBuf, PathBuf, PathBuf, PathBuf) = (
            root.as_ref().to_path_buf(),
            data.as_ref().to_path_buf(),
            config.as_ref().to_path_buf(),
            cwd.as_ref().to_path_buf(),
        );
        Self::new(root, data, config, cwd)
    }
}

impl PathProvider for TestPathProvider {
    fn data(&self) -> PathResult {
        Ok(self.root.join(self.data.clone()))
    }

    fn config(&self) -> PathResult {
        Ok(self.root.join(self.config.clone()))
    }

    fn cwd(&self) -> PathResult {
        Ok(self.root.join(self.cwd.clone()))
    }
}
