use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::{
    prelude::*,
    template_meta_data::{AllSerdeTemplateMetaData, AllTemplateMetaData},
    AbsoluteExistingPath,
};

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

pub fn template_meta_path() -> AppResult<PathBuf> {
    let data_path = data_path()?;
    let t = data_path.join(constants::TEMPLATE_META_FILE_NAME);
    Ok(t)
}

pub fn get_all_template_meta() -> AppResult<AllTemplateMetaData> {
    let path = template_meta_path()?;
    let exits_path = path
        .try_exists()
        .context("Could not find out if meta template file exits")?;
    let templates = if exits_path {
        let content = std::fs::read_to_string(path)
            .context("Could not read existing file for template meta")?;
        let read_templates: AllSerdeTemplateMetaData = serde_json::from_str(&content)?;
        let valid_templates: AllTemplateMetaData = read_templates.try_into()?;
        valid_templates
    } else {
        AllTemplateMetaData::default()
    };
    Ok(templates)
}

pub fn save_all_template(to_save: &AllTemplateMetaData) -> AppResult {
    let path = template_meta_path()?;

    let to_save = {
        let to_save_cloned = to_save.clone();
        let valid: AllSerdeTemplateMetaData = to_save_cloned.into();
        serde_json::to_string_pretty(&valid)
    }?;

    info!("Saving meta template to {:?}", path);
    std::fs::write(path, to_save)?;

    Ok(())
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
    let file_meta = std::fs::metadata(path.as_path())
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
