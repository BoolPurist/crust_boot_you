use std::path::{Path, PathBuf};

use fs_extra::dir::CopyOptions;

use crate::prelude::*;

use super::FileManipulator;
#[derive(Default, Debug)]
pub struct OsFileManipulator;

impl FileManipulator for OsFileManipulator {
    fn copy_file(&self, from: &Path, to: &Path) -> AppResult {
        std::fs::copy(from, to).context("failed to copy file to target location")?;
        debug!("Copied file {:?} to {:?}", from, to);
        Ok(())
    }

    fn copy_dir(&self, from: &Path, to: &Path) -> AppResult {
        _ = fs_extra::dir::copy(
            from,
            to,
            &CopyOptions::default().content_only(true).overwrite(true),
        )?;
        debug!("Copied files in folder {:?} to {:?}", from, to);
        Ok(())
    }

    fn ensure_dir(&self, location: &Path) -> AppResult {
        std::fs::create_dir_all(location).context("Could ensure a specific folder exits")?;
        debug!("Ensured that fodler {:?} exits", location);
        Ok(())
    }

    fn try_exits(&self, location: &Path) -> AppResult<bool> {
        location
            .try_exists()
            .context("Could determine if file exits")
    }

    fn list_first_level_dir(&self, location: &Path) -> AppResult<Vec<PathBuf>> {
        let mut directories: Vec<PathBuf> = Vec::new();
        for entry in std::fs::read_dir(location)? {
            let next = entry?;
            let path = next.path();
            match next.file_type() {
                Ok(file_type) if file_type.is_dir() => directories.push(path),
                Ok(_file) => debug!("File {:?} not listed as template", path),
                Err(error) => warn!("Could not determine type of {:?}.\nError: {}", path, error),
            }
        }
        Ok(directories)
    }
}
