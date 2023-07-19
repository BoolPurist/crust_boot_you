use fs_extra::dir::CopyOptions;

use crate::prelude::*;

use super::FileManipulator;
#[derive(Default, Debug)]
pub struct OsFileManipulator;

impl FileManipulator for OsFileManipulator {
    fn copy_file(
        &self,
        from: impl AsRef<std::path::Path>,
        to: impl AsRef<std::path::Path>,
    ) -> AppResult {
        let (to, from) = (to.as_ref(), from.as_ref());
        std::fs::copy(from, to).context("failed to copy file to target location")?;
        debug!("Copied file {:?} to {:?}", from, to);
        Ok(())
    }

    fn copy_dir(
        &self,
        from: impl AsRef<std::path::Path>,
        to: impl AsRef<std::path::Path>,
    ) -> AppResult {
        let (to, from) = (to.as_ref(), from.as_ref());
        _ = fs_extra::dir::copy(from, to, &CopyOptions::default().content_only(true))?;
        debug!("Copied files in folder {:?} to {:?}", from, to);
        Ok(())
    }

    fn ensure_dir(&self, location: impl AsRef<std::path::Path>) -> AppResult {
        let location = location.as_ref();
        std::fs::create_dir_all(location).context("Could ensure a specific folder exits")?;
        debug!("Ensured that fodler {:?} exits", location);
        Ok(())
    }
}
