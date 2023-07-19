use crate::prelude::*;
pub use os_file_manipulator::OsFileManipulator;
use std::path::Path;
mod os_file_manipulator;

#[cfg_attr(test, automock)]
pub trait FileManipulator {
    fn copy_file(&self, from: &Path, to: &Path) -> AppResult;
    fn copy_dir(&self, from: &Path, to: &Path) -> AppResult;
    fn ensure_dir(&self, location: &Path) -> AppResult;
    fn try_exits(&self, location: &Path) -> AppResult<bool>;
}
