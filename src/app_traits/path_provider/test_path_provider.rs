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
