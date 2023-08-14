use crate::prelude::*;
use derive_more::Into;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct AbsoluteExistingDirPath(PathBuf);

impl FromStr for AbsoluteExistingDirPath {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let existing: AbsoluteExistingPath = s.parse()?;
        Self::new(existing)
    }
}

impl AbsoluteExistingDirPath {
    pub fn new(absolute_existing: AbsoluteExistingPath) -> AppResult<Self> {
        let absolute_existing: PathBuf = absolute_existing.into();

        ensure!(
            absolute_existing.is_dir(),
            anyhow!(
                "Path does not point to a directory at {:?}",
                &absolute_existing
            )
        );
        Ok(Self(absolute_existing))
    }
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Debug, Clone, Into)]
pub struct AbsoluteExistingPath(PathBuf);

impl AbsoluteExistingPath {
    pub fn new(path: PathBuf, resolver: &impl PathResolver) -> AppResult<Self> {
        let new_p = resolver
            .absolute_and_exits(&path)?
            .ok_or_else(|| anyhow!("Path {:?} does not exit !.\n", path))?
            .to_path_buf();
        Ok(Self(new_p))
    }
}

impl FromStr for AbsoluteExistingPath {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s.into();
        Self::new(path, &*constants::USED_PATH_PROVIDER)
    }
}

impl std::ops::Deref for AbsoluteExistingPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRef<Path> for AbsoluteExistingPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
