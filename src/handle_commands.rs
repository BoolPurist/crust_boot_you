use std::path::{Path, PathBuf};

use fs_extra::dir::CopyOptions;

use crate::{
    cli::{SaveTemplateCli, TemplateCliArg},
    file_management::{self, FileKind},
    prelude::*,
    AbsoluteExistingPath, AppCliEntry, NotEmptyText, SubCommands,
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

    let path_to_template = file_management::specefic_template_path(name.as_ref())?;

    if !path_to_template.exists() {
        bail!("No content for teamplate named {}, could be found", name);
    }

    let cwd = file_management::try_get_cwd()?;
    copy_content_folder(path_to_template, &cwd)?;

    Ok(format!(
        "Folder {:?} filled with content from Template ({})",
        cwd, name
    ))
}

fn handle_save_template(from_cli: &SaveTemplateCli) -> ReturnType {
    debug!("Handling subcommand: {:?}", "SaveTemplate");
    let arguments = from_cli.arguments();
    let source_path = arguments.path();
    info!("Copying from {:?}", source_path);
    let file_type = file_management::detect_file_kind(source_path)?;

    match file_type {
        FileKind::File => {
            debug!("Detected {:?} as file", source_path);
            let ensured_template_folder = ensure_target_template_folder(arguments)?;

            let target_path = construct_file_target_path(source_path, &ensured_template_folder)?;
            debug!(
                "Copying from source path {:?} to target path {:?}",
                source_path, target_path,
            );
            std::fs::copy(source_path.as_ref(), target_path.as_path())
                .context("failed to copy file to target location")?;
        }
        FileKind::Folder => {
            debug!("Detected {:?} as folder", source_path);
            let target_path = ensure_target_template_folder(arguments)?;
            debug!(
                "Copying from source path {:?} to target path {:?}",
                source_path, target_path,
            );
            copy_content_folder(source_path, target_path)?;
        }
    }

    let msg_to_user = format!(
        "Created template with name ({}) from the {:?}",
        arguments.name(),
        source_path.as_ref(),
    );
    Ok(msg_to_user)
}

fn construct_file_target_path(
    source_path: &AbsoluteExistingPath,
    ensured_template_folder: &Path,
) -> AppResult<PathBuf> {
    let file_name = source_path
        .as_ref()
        .file_name()
        .ok_or(anyhow!("Could not extract file name"))?;

    let file_name = &Path::new(file_name);
    Ok(ensured_template_folder.join(file_name))
}

fn ensure_target_template_folder(arguments: &TemplateCliArg) -> AppResult<AbsoluteExistingPath> {
    let specific_template_folder =
        file_management::specefic_template_path(arguments.name().as_ref())?;

    file_management::ensure_path_exits(specific_template_folder)
}

fn copy_content_folder(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<u64, fs_extra::error::Error> {
    fs_extra::dir::copy(from, to, &CopyOptions::default().content_only(true))
}
