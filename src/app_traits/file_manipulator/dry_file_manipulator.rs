use crate::{file_management::FileNodeMeta, prelude::*};

use super::OsFileManipulator;
#[derive(Default)]
pub struct DryFileManipulator {
    os_imp: OsFileManipulator,
}
impl FileManipulator for DryFileManipulator {
    fn copy_file(&self, from: &Path, to: &Path) -> AppIoResult {
        let to_print = format!("Would copy file from {:?} to {:?}", from, to);
        crate::print_dry(&to_print);
        Ok(())
    }

    fn copy_dir(&self, from: &Path, to: &Path) -> AppIoResult {
        let to_print = format!(
            "Would copy content of directory from {:?} to {:?}",
            from, to
        );
        crate::print_dry(&to_print);
        Ok(())
    }

    fn ensure_dir(&self, location: &Path) -> AppIoResult {
        let to_print = format!(
            "Make sure that directory {:?} exits by creating it if not there",
            location
        );
        crate::print_dry(&to_print);
        Ok(())
    }

    fn try_exits(&self, location: &Path) -> AppIoResult<bool> {
        self.os_imp.try_exits(location)
    }

    fn list_first_level_dir(&self, location: &Path) -> AppIoResult<Vec<PathBuf>> {
        self.os_imp.list_first_level_dir(location)
    }

    fn all_nodes_inside(&self, location: &Path) -> AppIoResult<Vec<FileNodeMeta>> {
        self.os_imp.all_nodes_inside(location)
    }

    fn delete_whole_folder(&self, location: &Path) -> AppIoResult {
        let to_print = format!("Would delete the whole directory at {:?}", location);
        crate::print_dry(&to_print);
        Ok(())
    }

    fn write_file_to(&self, location: &Path, _content: &str) -> AppIoResult {
        let to_print = format!("Would write to file at {:?}", location);
        crate::print_dry(&to_print);
        Ok(())
    }
}
