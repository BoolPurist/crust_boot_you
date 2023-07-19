use crate::{
    cli::SaveTemplateCli,
    file_management::{self, FileKind},
    prelude::*,
    AppCliEntry, NotEmptyText, SubCommands,
};
use std::path::Path;

pub type ReturnType = AppResult<String>;

pub fn handle(
    path_provider: impl PathProvider,
    file_manipulator: impl FileManipulator,
    args: &AppCliEntry,
) -> ReturnType {
    match args.sub_commands() {
        SubCommands::LoadTemplate { name } => {
            handle_load_template(path_provider, file_manipulator, name)
        }
        SubCommands::SaveTemplate(args) => {
            handle_save_template(path_provider, file_manipulator, args)
        }
    }
}

fn handle_load_template(
    path_provider: impl PathProvider,
    file_manipulator: impl FileManipulator,
    name: &NotEmptyText,
) -> ReturnType {
    debug!("Handling subcommand: {:?}", "LoadTemplate");

    let path_to_template = path_provider.specific_entry_template_files(name)?;

    if !path_to_template.exists() {
        bail!("No content for teamplate named {}, could be found", name);
    }

    let cwd = path_provider.cwd()?;
    file_manipulator.copy_dir(path_to_template, &cwd)?;

    Ok(format!(
        "Folder {:?} filled with content from Template ({})",
        cwd, name
    ))
}

fn save_template(
    path_provider: impl PathProvider,
    file_manipulator: impl FileManipulator,
    name: &NotEmptyText,
    path: impl AsRef<Path>,
) -> ReturnType {
    debug!("Handling subcommand: {:?}", "SaveTemplate");
    let (source_path, template_name) = (path.as_ref(), name);
    info!("Copying from {:?}", source_path);
    let file_type = file_management::detect_file_kind(source_path)?;

    match file_type {
        FileKind::File => {
            debug!("Detected {:?} as file", source_path);
            let ensured_template_folder = file_management::ensure_target_template_folder(
                &path_provider,
                &file_manipulator,
                template_name,
            )?;

            let target_path =
                file_management::construct_file_target_path(source_path, &ensured_template_folder)?;
            debug!(
                "Copying from source path {:?} to target path {:?}",
                source_path, target_path,
            );
            file_manipulator.copy_file(source_path, target_path)?;
        }
        FileKind::Folder => {
            debug!("Detected {:?} as folder", source_path);
            let target_path = file_management::ensure_target_template_folder(
                &path_provider,
                &file_manipulator,
                template_name,
            )?;
            debug!(
                "Copying from source path {:?} to target path {:?}",
                source_path, target_path,
            );
            file_manipulator.copy_dir(source_path, target_path)?;
        }
    }

    let msg_to_user = format!(
        "Created template with name ({}) from the {:?}",
        name, source_path,
    );
    Ok(msg_to_user)
}
fn handle_save_template(
    path_provider: impl PathProvider,
    file_manipulator: impl FileManipulator,
    from_cli: &SaveTemplateCli,
) -> ReturnType {
    let arguments = from_cli.arguments();
    let (name, path) = (arguments.name(), arguments.path());
    save_template(path_provider, file_manipulator, name, path)
}
