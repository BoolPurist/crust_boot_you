use std::{fs::FileType, path::Path};

use crate::prelude::*;

pub use node_entry_meta::NodeEntryMeta;
pub use os_io_error::AppIoError;
pub use source_target_node::SourceTargetNode;

mod node_entry_meta;
mod os_io_error;
mod source_target_node;
pub mod write_transactions;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(test, derive(Serialize, Deserialize, PartialOrd, Ord))]
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
