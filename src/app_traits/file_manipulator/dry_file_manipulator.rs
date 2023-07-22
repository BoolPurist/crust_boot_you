use crate::{
    app_traits::path_resolver::OsPathResolver, file_management::NodeEntryMeta, prelude::*,
};

use super::OsFileManipulator;
#[derive(Default)]
pub struct DryFileManipulator {
    os_imp: OsFileManipulator,
}
impl FileManipulator for DryFileManipulator {
    type Resolver = OsPathResolver;

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

    fn list_first_level_dir(&self, location: &Path) -> AppIoResult<Vec<PathBuf>> {
        self.os_imp.list_first_level_dir(location)
    }

    fn delete_whole_folder(&self, location: &Path) -> AppIoResult {
        let to_print = format!("Would delete the whole directory at {:?}", location);
        crate::print_dry(&to_print);
        Ok(())
    }

    fn all_nodes_inside(&self, location: &Path) -> AppIoResult<Vec<NodeEntryMeta>> {
        self.os_imp.all_nodes_inside(location)
    }

    fn write_file_to(&self, location: &Path, _content: &str) -> AppIoResult {
        let to_print = format!("Would write to file at {:?}", location);
        crate::print_dry(&to_print);
        Ok(())
    }

    fn resolver(&self) -> &Self::Resolver {
        &self.os_imp.resolver()
    }

    fn cwd(&self) -> AppIoResult<PathBuf> {
        self.os_imp.cwd()
    }
}
