pub use dev_os_file_manipulator::DevOsFileManipulator;
pub use dry_file_manipulator::DryFileManipulator;
pub use os_file_manipulator::OsFileManipulator;

mod dev_os_file_manipulator;
mod dry_file_manipulator;
mod os_file_manipulator;

use crate::{
    file_management::{FileNodeMeta, LoadedNode},
    prelude::*,
};
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
    fn copy_file(&self, from: &Path, to: &Path) -> AppIoResult;
    fn copy_dir(&self, from: &Path, to: &Path) -> AppIoResult;
    fn ensure_dir(&self, location: &Path) -> AppIoResult;
    fn try_exits(&self, location: &Path) -> AppIoResult<bool>;
    fn list_first_level_dir(&self, location: &Path) -> AppIoResult<Vec<PathBuf>>;
    fn delete_whole_folder(&self, location: &Path) -> AppIoResult;
    fn all_nodes_inside(&self, location: &Path) -> AppIoResult<Vec<FileNodeMeta>>;
    fn write_file_to(&self, location: &Path, content: &str) -> AppIoResult;

    fn no_filled_folder_there(&self, location: &Path) -> AppIoResult<bool> {
        let exits = self.try_exits(location)?;
        if exits {
            let entries = self.list_first_level_dir(location)?;
            Ok(entries.is_empty())
        } else {
            Ok(true)
        }
    }

    fn write_node(&self, loaded: LoadedNode) -> AppIoResult {
        match loaded {
            LoadedNode::File { path, content } => self.write_file_to(&path, &content),
            LoadedNode::Folder { path } => self.ensure_dir(&path),
        }
    }
}
