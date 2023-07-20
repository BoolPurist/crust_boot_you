use crate::{file_management::FileNodeMeta, prelude::*};

use super::{panic_if_outside_tmp, OsFileManipulator};
#[derive(Debug, Default)]
pub struct DevOsFileManipulator {
    os_impl: OsFileManipulator,
}

impl DevOsFileManipulator {
    fn check_to_and_from(from: &Path, to: &Path) {
        panic_if_outside_tmp(from);
        panic_if_outside_tmp(to);
    }
}

impl FileManipulator for DevOsFileManipulator {
    fn copy_file(&self, from: &Path, to: &Path) -> AppIoResult {
        Self::check_to_and_from(from, to);
        self.os_impl.copy_file(from, to)
    }

    fn copy_dir(&self, from: &Path, to: &Path) -> AppIoResult {
        Self::check_to_and_from(from, to);
        self.os_impl.copy_dir(from, to)
    }

    fn ensure_dir(&self, location: &Path) -> AppIoResult {
        panic_if_outside_tmp(location);
        self.os_impl.ensure_dir(location)
    }

    fn try_exits(&self, location: &Path) -> AppIoResult<bool> {
        panic_if_outside_tmp(location);
        self.os_impl.try_exits(location)
    }

    fn list_first_level_dir(&self, location: &Path) -> AppIoResult<Vec<PathBuf>> {
        panic_if_outside_tmp(location);
        self.os_impl.list_first_level_dir(location)
    }

    fn all_nodes_at(&self, location: &Path) -> AppIoResult<Vec<FileNodeMeta>> {
        panic_if_outside_tmp(location);
        self.os_impl.all_nodes_at(location)
    }

    fn delete_whole_folder(&self, location: &Path) -> AppIoResult {
        panic_if_outside_tmp(location);
        self.os_impl.delete_whole_folder(location)
    }

    fn write_file_to(&self, location: &Path, content: &str) -> AppIoResult {
        panic_if_outside_tmp(location);
        self.os_impl.write_file_to(location, content)
    }
}
