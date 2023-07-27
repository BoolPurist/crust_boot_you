use crate::prelude::*;
use std::path::PathBuf;

use super::PathProvider;

#[derive(Debug)]
pub struct TestPathProvider {
    root: PathBuf,
    data: PathBuf,
    config: PathBuf,
}

impl TestPathProvider {
    pub fn new(root: PathBuf, data: PathBuf, config: PathBuf) -> Self {
        Self { root, data, config }
    }

    pub fn clone_from(
        root: impl AsRef<Path>,
        data: impl AsRef<Path>,
        config: impl AsRef<Path>,
    ) -> Self {
        let (root, data, config): (PathBuf, PathBuf, PathBuf) = (
            root.as_ref().to_path_buf(),
            data.as_ref().to_path_buf(),
            config.as_ref().to_path_buf(),
        );
        Self::new(root, data, config)
    }
}

impl PathProvider for TestPathProvider {
    fn logger_file_location(&self) -> PathResult {
        todo!();
    }
    fn data(&self) -> PathResult {
        Ok(self.root.join(self.data.clone()))
    }

    fn config(&self) -> PathResult {
        Ok(self.root.join(self.config.clone()))
    }

    fn logger_folder_location(&self) -> PathResult {
        todo!()
    }
}
