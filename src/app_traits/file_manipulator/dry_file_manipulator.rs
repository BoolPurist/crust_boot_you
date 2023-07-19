use crate::prelude::*;

use super::OsFileManipulator;
#[derive(Default)]
pub struct DryFileManipulator {
    os_imp: OsFileManipulator,
}
impl FileManipulator for DryFileManipulator {
    fn copy_file(&self, from: &Path, to: &Path) -> AppResult {
        let to_print = format!("Would copy file from {:?} to {:?}", from, to);
        crate::print_dry(&to_print);
        Ok(())
    }

    fn copy_dir(&self, from: &Path, to: &Path) -> AppResult {
        let to_print = format!(
            "Would copy content of directory from {:?} to {:?}",
            from, to
        );
        crate::print_dry(&to_print);
        Ok(())
    }

    fn ensure_dir(&self, location: &Path) -> AppResult {
        let to_print = format!(
            "Make sure that directory {:?} exits by creating it if not there",
            location
        );
        crate::print_dry(&to_print);
        Ok(())
    }

    fn try_exits(&self, location: &Path) -> AppResult<bool> {
        self.os_imp.try_exits(location)
    }

    fn list_first_level_dir(&self, location: &Path) -> AppResult<Vec<PathBuf>> {
        self.os_imp.list_first_level_dir(location)
    }
}
