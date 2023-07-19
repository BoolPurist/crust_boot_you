use crate::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AbsoluteExistingPath(PathBuf);
impl AbsoluteExistingPath {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let abs_path = std::fs::canonicalize(&path)
            .with_context(|| format!("{:?} could not be resolved", &path))?;
        Ok(Self(abs_path))
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
