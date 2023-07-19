use std::path::Path;

use crate::prelude::*;
pub use os_file_manipulator::OsFileManipulator;
mod os_file_manipulator;

pub trait FileManipulator {
    fn copy_file(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> AppResult;
    fn copy_dir(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> AppResult;
    fn ensure_dir(&self, location: impl AsRef<Path>) -> AppResult;
}
