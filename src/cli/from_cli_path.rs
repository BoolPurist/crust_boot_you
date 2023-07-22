use crate::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AbsoluteExistingPath(PathBuf);
impl AbsoluteExistingPath {
    pub fn new(path: PathBuf, resolver: &impl PathResolver) -> AppResult<Self> {
        let new_p = resolver
            .absolute_and_exits(&path)?
            .ok_or_else(|| anyhow!("Path {:?} does not exit !.\n", path))?
            .to_path_buf();
        Ok(Self(new_p))
        // return if cfg!(debug_assertions) {
        //     handle_dev_and_under_tmp(&path)
        // } else {
        //     match std::fs::canonicalize(&path) {
        //         Ok(resolved) => Ok(Self(resolved)),
        //         Err(may_not_found) if may_not_found.kind() == ErrorKind::NotFound => {
        //             return_not_found(&path)
        //         }
        //         Err(other_error) => {
        //             Err(other_error).with_context(|| format!("{:?} could not be resolved", &path))
        //         }
        //     }
        // };
        //
        // fn return_not_found(path: &Path) -> AppResult<AbsoluteExistingPath> {
        //     Err(anyhow!("Path {:?} does not exit !.\n", path))
        // }
        //
        // fn handle_dev_and_under_tmp(path: &Path) -> AppResult<AbsoluteExistingPath> {
        //     use path_absolutize::*;
        //     let dev_paths = DevPathProvider::default();
        //     let cwd = dev_paths
        //         .cwd()
        //         .expect("Should possible to access cwd during development");
        //     let absolute = path.absolutize_virtually(cwd)?;
        //     match absolute.try_exists() {
        //         Ok(true) => Ok(Self(absolute.to_path_buf())),
        //         Ok(false) => return_not_found(path),
        //         Err(error) => panic!(
        //             "During development can not determine path {:?} exits.\nError: {:?}",
        //             path, error
        //         ),
        //     }
        // }
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
