use std::{fs::FileType, path::Path};

use anyhow::Context;

use crate::prelude::*;

pub use file_node::FileNodeMeta;
pub use loaded_node::LoadedNode;
pub use os_io_error::AppIoError;
pub use source_target_node::SourceTargetNode;

mod file_node;
mod loaded_node;
mod os_io_error;
mod source_target_node;

#[derive(Debug, PartialEq, Eq)]
pub enum FileKind {
    File,
    Folder,
    Symlink,
}

impl TryFrom<FileType> for FileKind {
    type Error = AppIoError;
    fn try_from(value: FileType) -> Result<Self, Self::Error> {
        let os_file_type = value;
        if os_file_type.is_dir() {
            Ok(FileKind::Folder)
        } else if os_file_type.is_file() {
            Ok(FileKind::File)
        } else if os_file_type.is_symlink() {
            Ok(FileKind::Symlink)
        } else {
            Err(AppIoError::Custom(format!(
                "File type {:?} is not supported",
                os_file_type
            )))
        }
    }
}

pub fn requires_as_folder(location: &Path) -> AppIoResult {
    let is_root_a_folder = detect_file_kind(location)
        .map(|file_t| file_t == FileKind::Folder)
        .unwrap_or(false);
    if !is_root_a_folder {
        return Err(AppIoError::custom(format!("{:?} is not folder", location)));
    }
    Ok(())
}

pub fn detect_file_kind(path: &Path) -> AppIoResult<FileKind> {
    let file_meta = std::fs::metadata(path)?;

    let os_file_type = file_meta.file_type();
    os_file_type.try_into()
}

pub fn construct_file_target_path(
    source_path: &Path,
    ensured_template_folder: &Path,
) -> PathResult {
    let file_name = source_path.file_name().ok_or(anyhow!(
        "Could not extract file name from {:?}",
        source_path
    ))?;

    let file_name = &Path::new(file_name);
    Ok(ensured_template_folder.join(file_name))
}

pub fn ensure_target_template_folder(
    path_provider: &impl PathProvider,
    file_manipulator: &impl FileManipulator,
    template_name: &NotEmptyText,
) -> PathResult {
    let specific_template_folder = path_provider.specific_entry_template_files(template_name)?;
    file_manipulator.ensure_dir(&specific_template_folder)?;
    Ok(specific_template_folder)
}
