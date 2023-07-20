use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;

use crate::{
    file_management::{FileKind, FileNodeMeta},
    prelude::*,
};

use super::FileManipulator;

#[derive(Default, Debug)]
pub struct OsFileManipulator;

impl FileManipulator for OsFileManipulator {
    fn copy_file(&self, from: &Path, to: &Path) -> AppIoResult {
        std::fs::copy(from, to)?;
        debug!("Copied file {:?} to {:?}", from, to);
        Ok(())
    }

    fn copy_dir(&self, from: &Path, to: &Path) -> AppIoResult {
        _ = fs_extra::dir::copy(
            from,
            to,
            &CopyOptions::default().content_only(true).overwrite(true),
        )?;
        debug!("Copied files in folder {:?} to {:?}", from, to);
        Ok(())
    }

    fn ensure_dir(&self, location: &Path) -> AppIoResult {
        std::fs::create_dir_all(location)?;
        debug!("Ensured that folder {:?} exits", location);
        Ok(())
    }

    fn try_exits(&self, location: &Path) -> AppIoResult<bool> {
        let exits = location.try_exists()?;
        Ok(exits)
    }

    fn list_first_level_dir(&self, location: &Path) -> AppIoResult<Vec<PathBuf>> {
        let mut directories: Vec<PathBuf> = Vec::new();
        for entry in std::fs::read_dir(location)? {
            let next = entry?;
            let path = next.path();
            match next.file_type() {
                Ok(file_type) if file_type.is_dir() => directories.push(path),
                Ok(_file) => debug!("File/Symlink {:?} not listed as template", path),
                Err(error) => warn!("Could not determine type of {:?}.\nError: {}", path, error),
            }
        }
        Ok(directories)
    }

    fn all_nodes_inside(&self, location: &Path) -> AppIoResult<Vec<FileNodeMeta>> {
        let mut to_return: Vec<FileNodeMeta> = Vec::new();
        let mut buffer: VecDeque<FileNodeMeta> = Default::default();
        walk_level_of(location, &mut buffer)?;
        while let Some(next) = buffer.pop_front() {
            if *next.node_type() == FileKind::Folder {
                walk_level_of(next.source_path(), &mut buffer)?;
            }
            to_return.push(next);
        }

        return Ok(to_return);

        fn walk_level_of(path: &Path, buffer: &mut VecDeque<FileNodeMeta>) -> AppIoResult {
            for entry in std::fs::read_dir(path)? {
                let next = entry?;
                let file_kind: FileKind = next.file_type()?.try_into()?;
                let path = next.path();
                let file_node = FileNodeMeta::new(file_kind, path);
                buffer.push_back(file_node);
            }
            Ok(())
        }
    }

    fn delete_whole_folder(&self, location: &Path) -> AppIoResult {
        dbg!(location);
        std::fs::remove_dir_all(location)?;
        info!("Deleted folder with all its content at {:?}", location);
        Ok(())
    }

    fn write_file_to(&self, location: &Path, content: &str) -> AppIoResult {
        debug!(
            "Wrote to file folder with all its content at {:?}",
            location
        );
        std::fs::write(location, content)?;
        Ok(())
    }
}
