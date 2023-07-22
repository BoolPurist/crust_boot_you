use crate::prelude::*;

use super::PathResolver;
#[derive(Default)]
pub struct TestPathResolver {
    root: PathBuf,
    exits: bool,
}

impl PathResolver for TestPathResolver {
    fn root(&self) -> &Path {
        self.root.as_path()
    }

    fn try_exits(&self, _path: &Path) -> AppIoResult<bool> {
        Ok(self.exits)
    }
}

impl TestPathResolver {
    pub fn new(root: PathBuf, exits: bool) -> Self {
        Self { root, exits }
    }
}
