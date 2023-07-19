use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::{prelude::*, AbsoluteExistingPath};

pub fn data_path() -> AppResult<PathBuf> {
    let data_path = if cfg!(debug_assertions) {
        constants::PROJECT_ROOT_PATH
            .join(constants::dev::ENTRY_FOLDER)
            .join(constants::dev::DATA_FOLDER)
    } else {
        todo!("Not implemented for prodution so far.")
    };

    debug!("Data path: {:?}", data_path);
    Ok(data_path)
}

pub fn data_path_templates() -> AppResult<PathBuf> {
    let data_path_folder = {
        let data_path = data_path()?;
        if cfg!(debug_assertions) {
            data_path.join(constants::TEMPLATES_FOLDER)
        } else {
            todo!("Not implemented for prodution so far.")
        }
    };

    debug!("Template path: {:?}", data_path_folder);

    Ok(data_path_folder)
}

pub fn specefic_template_path(name: impl AsRef<Path>) -> AppResult<PathBuf> {
    let template_path = data_path_templates()?;
    let target_folder = template_path.join(name.as_ref());
    Ok(target_folder)
}

pub fn ensure_path_exits(path: PathBuf) -> AppResult<AbsoluteExistingPath> {
    debug!("Ensures that path {:?} exits", path);
    dbg!();
    std::fs::create_dir_all(&path)?;
    let abs_path = AbsoluteExistingPath::new(path)?;
    Ok(abs_path)
}

pub enum FileKind {
    File,
    Folder,
}

pub fn detect_file_kind(path: &AbsoluteExistingPath) -> AppResult<FileKind> {
    let file_meta = std::fs::metadata(path.as_ref())
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

pub fn try_get_cwd() -> AppResult<PathBuf> {
    std::env::current_dir().context("Could not retrieve current working directory")
}
