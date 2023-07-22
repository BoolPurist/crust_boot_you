use crate::{app_traits::path_provider, prelude::*};
#[derive(Debug)]
pub struct DevPathResolver {
    root: PathBuf,
}

impl DevPathResolver {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl Default for DevPathResolver {
    fn default() -> Self {
        Self {
            root: path_provider::get_root_dev(),
        }
    }
}

impl PathResolver for DevPathResolver {
    fn try_exits(&self, path: &Path) -> AppIoResult<bool> {
        let exits = path.try_exists()?;
        Ok(exits)
    }

    fn root(&self) -> &Path {
        self.root.as_path()
    }
}
