use std::path::Path;

use anyhow::Context;

use crate::prelude::*;

pub enum FileKind {
    File,
    Folder,
}

pub fn detect_file_kind(path: impl AsRef<Path>) -> AppResult<FileKind> {
    let path = path.as_ref();
    let file_meta = std::fs::metadata(path)
        .with_context(|| format!("Could extract meta data from {:?}", path))?;

    let os_file_type = file_meta.file_type();
    if os_file_type.is_dir() {
        Ok(FileKind::Folder)
    } else if os_file_type.is_file() {
        Ok(FileKind::File)
    } else {
        Err(anyhow!("File type {:?} is not supported", os_file_type))
    }
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
