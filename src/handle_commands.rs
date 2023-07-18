use std::path::{Path, PathBuf};

use fs_extra::dir::CopyOptions;

use crate::{
    cli::{SaveTemplateCli, TemplateCliArg},
    file_management::{self, FileKind},
    prelude::*,
    template_meta_data::TemplateMeta,
    AbsoluteExistingPath, AppCliEntry, NotEmptyText, SubCommands, Template,
};
pub type ReturnType = AppResult<String>;
pub fn handle(args: &AppCliEntry) -> ReturnType {
    match args.sub_commands() {
        SubCommands::LoadTemplate { name } => handle_create_load_template(name),
        SubCommands::SaveTemplate(args) => handle_save_template(args),
    }
}

fn handle_create_load_template(name: &NotEmptyText) -> ReturnType {
    debug!("Handling subcommand: {:?}", "LoadTemplate");
    let loaded_template = file_management::get_all_template_meta()?;

    // Check if tempalte  extis
    if loaded_template.get_template(name.as_ref()).is_none() {
        bail!("Template with name {} is not registered", name);
    }

    // Get path to template folder
    let path_to_template = file_management::specefic_template_path(name.as_ref())?;

    if !path_to_template.exists() {
        bail!("No content for teamplate named {}, could be found", name);
    }

    // Get current cwd
    let cwd = file_management::try_get_cwd()?;
    fs_extra::copy_items(&[path_to_template], &cwd, &CopyOptions::default())?;

    bail!("");
}

fn handle_save_template(from_cli: &SaveTemplateCli) -> ReturnType {
    debug!("Handling subcommand: {:?}", "SaveTemplate");
    let arguments = from_cli.arguments();
    let source_path = arguments.path();
    info!("Copying from {:?}", source_path);
    let file_type = file_management::detect_file_kind(source_path)?;

    let mut meta = file_management::get_all_template_meta()?;
    let name = arguments.name();
    if let Some(_) = meta.get_template(name.as_ref()) {
        bail!("Template with with name {} already extis", name);
    }

    match file_type {
        FileKind::File => {
            debug!("Detected {:?} as file", source_path);
            let target_path = construct_and_ensure_targe_location(source_path, arguments)?;
            debug!(
                "Copying from source path {:?} to target path {:?}",
                source_path, target_path,
            );
            std::fs::copy(source_path.as_path(), target_path.as_path())
                .context("failed to copy file to target location")?;
        }
        FileKind::Folder => {
            debug!("Detected {:?} as folder", source_path);
            todo!("Copying folder is not supported yet !")
        }
    }

    let new = TemplateMeta::new(arguments.name().clone());
    meta.insert_template(new)?;
    file_management::save_all_template(&meta)?;

    let msg_to_user = format!(
        "Created template with name ({}) from the {:?}",
        arguments.name(),
        source_path.as_path(),
    );
    Ok(msg_to_user)
}

fn construct_and_ensure_targe_location(
    source_path: &AbsoluteExistingPath,
    arguments: &TemplateCliArg,
) -> AppResult<PathBuf> {
    let specific_template_folder =
        file_management::specefic_template_path(arguments.name().as_ref())?;

    let file_name = source_path
        .as_path()
        .file_name()
        .ok_or(anyhow!("Could not extract file name"))?;

    let file_name = &Path::new(file_name);
    let target_path = specific_template_folder.join(file_name);

    _ = file_management::ensure_path_exits(specific_template_folder)?;
    Ok(target_path)
}
