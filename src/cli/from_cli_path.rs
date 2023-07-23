use crate::{prelude::*, UsedPathResolver};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Clone)]
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
        let resolver = UsedPathResolver::default();
        let path = s.into();
        Self::new(path, &resolver)
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
