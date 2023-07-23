use super::PathResolver;
use crate::prelude::*;

#[derive(Debug, Default)]
pub struct OsPathResolver;

impl PathResolver for OsPathResolver {
    fn try_exits(&self, path: &std::path::Path) -> AppIoResult<bool> {
        let exits = path.try_exists()?;
        Ok(exits)
    }

    fn root(&self) -> &Path {
        Path::new(std::path::MAIN_SEPARATOR_STR)
    }
    fn absolute<'a>(&self, path: &'a Path) -> AppIoResult<std::borrow::Cow<'a, Path>> {
        use path_absolutize::Absolutize;
        let abs = path.absolutize()?;
        Ok(abs)
    }
}
