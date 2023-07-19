use crate::prelude::*;
use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct AbsoluteExistingPath(PathBuf);
impl AbsoluteExistingPath {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        match std::fs::canonicalize(&path) {
            Ok(resolved) => Ok(Self(resolved)),
            Err(may_not_found) if may_not_found.kind() == ErrorKind::NotFound => {
                Err(anyhow!("Path {:?} does not exit !.\n", &path))
            }
            Err(other_error) => {
                Err(other_error).with_context(|| format!("{:?} could not be resolved", &path))
            }
        }
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
