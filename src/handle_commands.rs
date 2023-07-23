use crate::{
    cli::{InitKind, LoadTemplateArg, SaveTemplateCli},
    file_management::{self, FileKind},
    prelude::*,
    AppCliEntry, SubCommands, ValidTemplateName,
};
use std::path::Path;

fn save_err_already_created_template(name: &ValidTemplateName) -> String {
    format!("Template with the name ({}) is already created", name)
}

pub fn handle(
    path_provider: &impl PathProvider,
    file_manipulator: &impl FileManipulator,
    args: &AppCliEntry,
) -> ReturnToUser {
    match args.sub_commands() {
        SubCommands::LoadTemplate(args) => {
            handle_load_template(path_provider, file_manipulator, args)
        }
        SubCommands::SaveTemplate(args) => {
            handle_save_template(path_provider, file_manipulator, args)
        }
        SubCommands::ListTemplate => handle_list_template(path_provider, file_manipulator),
        SubCommands::DeleteTemplate { name } => {
            handle_delete_template(path_provider, file_manipulator, name)
        }
    }
}

pub fn handle_delete_template(
    path_provider: &impl PathProvider,
    file_manipulator: &impl FileManipulator,
    name: &ValidTemplateName,
) -> ReturnToUser {
    let path_to_delete = path_provider.specific_entry_template(name)?;
    match file_manipulator.delete_whole_folder(&path_to_delete) {
        Ok(_) => {
            let message = success_delete_msg(name);
            Ok(message)
        }
        Err(AppIoError::NotFound) => bail!(error_delete_msg_not_found(name)),
        Err(error) => bail!(error_delet_msg_other_err(name, error)),
    }
}

fn success_delete_msg(name: &ValidTemplateName) -> String {
    format!("Template ({}) was deleted.", name.as_ref())
}
fn error_delete_msg_not_found(name: &ValidTemplateName) -> String {
    format!(
        "There is no template to be deleted with the name ({}).",
        name
    )
}

fn error_delet_msg_other_err(name: &ValidTemplateName, error: AppIoError) -> String {
    format!(
        "Could not delete template ({}) because of an error.\n{}",
        name, error
    )
}

pub fn handle_list_template(
    path_provider: &impl PathProvider,
    file_manipulator: &impl FileManipulator,
) -> ReturnToUser {
    let entry_point = path_provider.general_template_entry()?;
    file_manipulator.ensure_dir(&entry_point)?;
    let all_template_paths = file_manipulator.list_first_level_dir(&entry_point)?;

    let lines: String = all_template_paths
        .into_iter()
        .map(|path| {
            let template_name = path.file_name().unwrap().to_string_lossy();
            format!("{}  {:?}\n", template_name, path)
        })
        .collect();
    let output = if lines.is_empty() {
        "No templates created yet.".to_string()
    } else {
        let mut output = format!("{}\n", constants::TITLE_LIST_RESULT);
        output.push_str(&lines);
        output
    };

    Ok(output)
}

pub fn handle_load_template(
    path_provider: &impl PathProvider,
    file_manipulator: &impl FileManipulator,
    load_args: &LoadTemplateArg,
) -> ReturnToUser {
    debug!("Handling subcommand: {:?}", "LoadTemplate");
    let (name, init_kind) = (load_args.name(), load_args.with());

    let path_to_template = path_provider.specific_entry_template_files(name)?;

    match file_manipulator.try_exits(path_to_template.as_path()) {
        Ok(false) => bail!("No content for teamplate named {}, could be found", name),
        Err(error) => bail!(
            "No content for teamplate named {}, could be found because of error:\n {}",
            name,
            error
        ),
        _ => (),
    }

    let cwd = try_return_valid_target(file_manipulator, init_kind)?;
    file_manipulator
        .copy_dir(&path_to_template, &cwd)
        .with_context(|| format!("Failed to copy from {:?} to {:?}", path_to_template, cwd))?;

    return Ok(format!(
        "Folder {:?} filled with content from Template ({})",
        cwd, name
    ));

    fn try_return_valid_target(
        file_manipulator: &impl FileManipulator,
        _init_kind: InitKind,
    ) -> AppResult<PathBuf> {
        let cwd = file_manipulator
            .cwd()
            .context("Can not access current working directory. No target to copy to")?;

        let is_empty = file_manipulator.all_nodes_inside(&cwd)?.is_empty();
        if !is_empty {
            bail!(
                "Folder is not empty at {:?}. This is an error for init kind OnlyEmpty.",
                cwd
            );
        }
        Ok(cwd)
    }
}

pub fn handle_save_template(
    path_provider: &impl PathProvider,
    file_manipulator: &impl FileManipulator,
    from_cli: &SaveTemplateCli,
) -> ReturnToUser {
    let (name, path) = (from_cli.name(), from_cli.path());
    save_template(
        path_provider,
        file_manipulator,
        file_management::detect_file_kind,
        name,
        path,
    )
}

fn success_save_msg(name: &ValidTemplateName, file_kind: &str, source_path: &Path) -> String {
    format!(
        "Created template with name ({}) from the {} {:?}",
        name, file_kind, source_path,
    )
}
/// Creates tempate with its file inside of data folde of app.
/// Note: If executed for already saved template folder then file may be overriden in the template
/// by new files with the same name
///
/// # Errors
///
/// - If template folder not could be generated
/// - If file type of source path could be detected. Example: source path is a symlink
/// - If the copy process was not successful.
///
fn save_template(
    path_provider: &impl PathProvider,
    file_manipulator: &impl FileManipulator,
    on_detect_file_kind: impl Fn(&Path) -> AppIoResult<FileKind>,
    name: &ValidTemplateName,
    source_path: impl AsRef<Path>,
) -> ReturnToUser {
    debug!("Handling subcommand: {:?}", "SaveTemplate");
    let (source_path, template_name) = (source_path.as_ref(), name);
    info!("Copying from {:?}", source_path);

    {
        let template_path = path_provider.specific_entry_template(template_name)?;

        let exits = file_manipulator.try_exits(&template_path)?;
        if exits {
            bail!(save_err_already_created_template(template_name));
        }
    }

    let file_type = on_detect_file_kind(source_path)?;

    let path_to_target_files = path_provider.specific_entry_template_files(template_name)?;

    let file_kind = match file_type {
        FileKind::File => {
            handle_file(file_manipulator, &path_to_target_files, source_path)?;
            "file"
        }
        FileKind::Folder => {
            handle_dir(file_manipulator, &path_to_target_files, source_path)?;
            "folder"
        }
        FileKind::Symlink => {
            bail!("A symlink as a base for a template is not valid !")
        }
    };

    let msg_to_user = success_save_msg(name, file_kind, source_path);

    return Ok(msg_to_user);

    fn handle_dir(
        file_manipulator: &impl FileManipulator,
        target_path: &Path,
        source_path: &Path,
    ) -> AppResult {
        debug!("Detected {:?} as folder", source_path);
        file_manipulator.ensure_dir(target_path)?;
        debug!(
            "Copying from source path {:?} to target path {:?}",
            source_path, target_path,
        );
        file_manipulator.copy_dir(source_path, target_path)?;
        Ok(())
    }

    fn handle_file(
        file_manipulator: &impl FileManipulator,
        template_path: &Path,
        source_path: &Path,
    ) -> AppResult {
        debug!("Detected {:?} as file", source_path);
        file_manipulator.ensure_dir(template_path)?;

        let target_path = file_management::construct_file_target_path(source_path, template_path)?;
        debug!(
            "Copying from source path {:?} to target path {:?}",
            source_path, target_path,
        );
        file_manipulator.copy_file(source_path, &target_path)?;
        Ok(())
    }
}
