use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;

use crate::{
    app_traits::path_resolver::OsPathResolver,
    file_management::{FileKind, NodeEntryMeta},
    prelude::*,
};

use super::FileManipulator;

#[derive(Default, Debug)]
pub struct OsFileManipulator {
    resolver: OsPathResolver,
}

impl FileManipulator for OsFileManipulator {
    type Resolver = OsPathResolver;

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

    fn list_first_level_dir(&self, location: &Path) -> AppIoResult<Vec<PathBuf>> {
        let mut directories: Vec<PathBuf> = Vec::new();
        for entry in std::fs::read_dir(location)? {
            let next = entry?;
            let path = next.path();
            match next.file_type() {
                Ok(file_type) if file_type.is_dir() => directories.push(path),
                Ok(_file) => debug!("File|Symlink {:?} not listed as template", path),
                Err(error) => warn!("Could not determine type of {:?}.\nError: {}", path, error),
            }
        }
        Ok(directories)
    }

    fn all_nodes_inside(&self, location: &Path) -> AppIoResult<Vec<NodeEntryMeta>> {
        let mut to_return: Vec<NodeEntryMeta> = Vec::new();
        let mut buffer: VecDeque<NodeEntryMeta> = Default::default();
        walk_level_of(location, &mut buffer)?;
        while let Some(next) = buffer.pop_front() {
            if *next.node_type() == FileKind::Folder {
                walk_level_of(next.source_path(), &mut buffer)?;
            }
            to_return.push(next);
        }

        return Ok(to_return);

        fn walk_level_of(path: &Path, buffer: &mut VecDeque<NodeEntryMeta>) -> AppIoResult {
            for entry in std::fs::read_dir(path)? {
                let next = entry?;
                let file_kind: FileKind = next.file_type()?.try_into()?;
                let path = next.path();
                let file_node = NodeEntryMeta::new(file_kind, path);
                buffer.push_back(file_node);
            }
            Ok(())
        }
    }

    fn delete_whole_folder(&self, location: &Path) -> AppIoResult {
        std::fs::remove_dir_all(location)?;
        info!("Deleted folder with all its content at {:?}", location);
        Ok(())
    }

    fn write_file_to(&self, location: &Path, content: &str) -> AppIoResult {
        debug!("Wrote to file with all its content at {:?}", location);
        std::fs::write(location, content)?;
        Ok(())
    }

    fn resolver(&self) -> &Self::Resolver {
        &self.resolver
    }

    fn cwd(&self) -> AppIoResult<PathBuf> {
        let path = std::env::current_dir()?;
        Ok(path)
    }

    fn read_file(&self, path: &Path) -> AppIoResult<String> {
        let result = std::fs::read_to_string(path)?;
        debug!("Read content of file from {:?}", path);
        Ok(result)
    }
}
