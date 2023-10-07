use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::prelude::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct NonFilePath(PathBuf);

impl FromStr for NonFilePath {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        Self::new(path)
    }
}

impl NonFilePath {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let exits = path.try_exists()?;
        ensure!(
            exits && !path.is_file(),
            anyhow!("Path must not point to {:?} a file", &path)
        );
        Ok(Self(path))
    }
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}
