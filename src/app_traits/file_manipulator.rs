pub use dev_os_file_manipulator::DevOsFileManipulator;
pub use dry_file_manipulator::DryFileManipulator;
pub use os_file_manipulator::OsFileManipulator;

mod dev_os_file_manipulator;
mod dry_file_manipulator;
mod os_file_manipulator;

use crate::{file_management::FileNode, prelude::*};
use std::path::{Path, PathBuf};

use super::path_provider::get_root_dev;

pub fn panic_if_outside_tmp(path: &Path) {
    let dev_root = get_root_dev();
    if !path.starts_with(get_root_dev()) {
        panic!(
            "Path {:?} is outside of temp folder root {:?}.\n This is not allowed during development",
            path, dev_root
        );
    }
}

#[cfg_attr(test, automock)]
pub trait FileManipulator {
    fn copy_file(&self, from: &Path, to: &Path) -> AppResult;
    fn copy_dir(&self, from: &Path, to: &Path) -> AppResult;
    fn ensure_dir(&self, location: &Path) -> AppResult;
    fn try_exits(&self, location: &Path) -> AppResult<bool>;
    fn list_first_level_dir(&self, location: &Path) -> AppResult<Vec<PathBuf>>;
    fn is_existing_folder_empty(&self, location: &Path) -> AppResult<bool> {
        let entries = self.list_first_level_dir(location)?;
        Ok(entries.is_empty())
    }
    fn all_nodes_at(&self, location: &Path) -> AppResult<Vec<FileNode>>;
}
