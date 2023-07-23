pub use dev_os_file_manipulator::DevOsFileManipulator;
pub use dry_file_manipulator::DryFileManipulator;
pub use os_file_manipulator::OsFileManipulator;

mod dev_os_file_manipulator;
mod dry_file_manipulator;
mod os_file_manipulator;

use crate::{
    file_management::{LoadedNode, NodeEntryMeta},
    prelude::*,
};

use std::path::{Path, PathBuf};

pub trait FileManipulator {
    type Resolver: PathResolver;

    fn cwd(&self) -> AppIoResult<PathBuf>;
    fn copy_file(&self, from: &Path, to: &Path) -> AppIoResult;
    fn copy_dir(&self, from: &Path, to: &Path) -> AppIoResult;
    fn ensure_dir(&self, location: &Path) -> AppIoResult;
    fn list_first_level_dir(&self, location: &Path) -> AppIoResult<Vec<PathBuf>>;
    fn delete_whole_folder(&self, location: &Path) -> AppIoResult;
    fn all_nodes_inside(&self, location: &Path) -> AppIoResult<Vec<NodeEntryMeta>>;
    fn write_file_to(&self, location: &Path, content: &str) -> AppIoResult;
    fn resolver(&self) -> &Self::Resolver;

    fn root(&self) -> &Path {
        self.resolver().root()
    }
    fn try_exits(&self, location: &Path) -> AppIoResult<bool> {
        self.resolver().try_exits(location)
    }

    fn no_filled_folder_there(&self, location: &Path) -> AppIoResult<bool> {
        let exits = self.try_exits(location)?;
        if exits {
            let entries = self.list_first_level_dir(location)?;
            Ok(!entries.is_empty())
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
