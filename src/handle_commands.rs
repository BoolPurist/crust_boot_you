use crate::{
    cli::SaveTemplateCli,
    file_management::{self, FileKind},
    prelude::*,
    AppCliEntry, NotEmptyText, SubCommands,
};
use std::path::Path;

pub fn handle(
    path_provider: impl PathProvider,
    file_manipulator: impl FileManipulator,
    args: &AppCliEntry,
) -> ReturnToUser {
    match args.sub_commands() {
        SubCommands::LoadTemplate { name } => {
            handle_load_template(path_provider, file_manipulator, name)
        }
        SubCommands::SaveTemplate(args) => {
            handle_save_template(path_provider, file_manipulator, args)
        }
        SubCommands::ListTemplate => handle_list_template(path_provider, file_manipulator),
    }
}

fn handle_list_template(
    path_provider: impl PathProvider,
    file_manipulator: impl FileManipulator,
) -> ReturnToUser {
    let entry_point = path_provider.general_template_entry()?;
    file_manipulator.ensure_dir(&entry_point)?;
    let all_template_paths = file_manipulator.list_first_level_dir(&entry_point)?;

    let mut output = format!("{}\n", constants::TITLE_LIST_RESULT);
    let lines: String = all_template_paths
        .into_iter()
        .map(|path| {
            let line = path.file_name().unwrap().to_string_lossy();
            format!("{}\n", line)
        })
        .collect();
    output.push_str(&lines);
    Ok(output)
}

fn handle_load_template(
    path_provider: impl PathProvider,
    file_manipulator: impl FileManipulator,
    name: &NotEmptyText,
) -> ReturnToUser {
    debug!("Handling subcommand: {:?}", "LoadTemplate");

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

    let cwd = path_provider.cwd()?;
    file_manipulator.copy_dir(&path_to_template, &cwd)?;

    Ok(format!(
        "Folder {:?} filled with content from Template ({})",
        cwd, name
    ))
}

fn handle_save_template(
    path_provider: impl PathProvider,
    file_manipulator: impl FileManipulator,
    from_cli: &SaveTemplateCli,
) -> ReturnToUser {
    let arguments = from_cli.arguments();
    let (name, path) = (arguments.name(), arguments.path());
    save_template(
        path_provider,
        file_manipulator,
        file_management::detect_file_kind,
        name,
        path,
    )
}

/// Creates tempate with its file inside of data folde of app.
/// Note: If executed for already saved template folder then file may be overriden in the template
/// by new files with the same name
///
/// # Errors
///
/// - If template folder could be generated
/// - If file type of source path could be detected. Example: source path is a symlink
/// - If the copy process was not successful.
///
fn save_template(
    path_provider: impl PathProvider,
    file_manipulator: impl FileManipulator,
    on_detect_file_kind: impl Fn(&Path) -> AppResult<FileKind>,
    name: &NotEmptyText,
    source_path: impl AsRef<Path>,
) -> ReturnToUser {
    debug!("Handling subcommand: {:?}", "SaveTemplate");
    let (source_path, template_name) = (source_path.as_ref(), name);
    info!("Copying from {:?}", source_path);

    let file_type = on_detect_file_kind(source_path)?;

    let file_kind = match file_type {
        FileKind::File => {
            handle_file(path_provider, file_manipulator, template_name, source_path)?;
            "file"
        }
        FileKind::Folder => {
            handle_dir(path_provider, file_manipulator, template_name, source_path)?;
            "folder"
        }
    };

    let msg_to_user = format!(
        "Created template with name ({}) from the {} {:?}",
        name, file_kind, source_path,
    );

    return Ok(msg_to_user);

    fn handle_dir(
        path_provider: impl PathProvider,
        file_manipulator: impl FileManipulator,
        template_name: &NotEmptyText,
        source_path: &Path,
    ) -> AppResult {
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
        file_manipulator.copy_dir(source_path, &target_path)
    }

    fn handle_file(
        path_provider: impl PathProvider,
        file_manipulator: impl FileManipulator,
        template_name: &NotEmptyText,
        source_path: &Path,
    ) -> AppResult {
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
        file_manipulator.copy_file(source_path, &target_path)
    }
}

#[cfg(test)]

mod testing {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    use crate::app_traits::{
        file_manipulator::MockFileManipulator,
        path_provider::{MockPathProvider, TestPathProvider},
    };

    #[test]
    fn create_template_from_one_file() {
        let data_folder = PathBuf::from("user").join("data");
        let name: NotEmptyText = NotEmptyText::new("Some_Name".to_owned()).unwrap();
        let on_detect_file_kind = |_: &_| Ok(FileKind::File);
        let source_path = PathBuf::from("some/source");
        let paths =
            TestPathProvider::new(data_folder.clone(), PathBuf::from("a"), PathBuf::from("a"));
        let expected_ensured_template_folder = paths.specific_entry_template_files(&name).unwrap();
        let expected_target_folder = expected_ensured_template_folder.join("source");
        let expected_error_message =
            "Created template with name (Some_Name) from the file \"some/source\"".to_string();

        let file_manipulator = {
            let mut file_manipulator = MockFileManipulator::default();
            file_manipulator
                .expect_ensure_dir()
                .times(1)
                .withf(move |param| param == expected_ensured_template_folder)
                .returning(|_| Ok(()));

            let expected_source_path = source_path.clone();
            file_manipulator
                .expect_copy_file()
                .times(1)
                .withf(move |actual_source_path, actual_target_folder| {
                    actual_source_path == expected_source_path
                        && actual_target_folder == expected_target_folder
                })
                .returning(|_, _| Ok(()));

            file_manipulator
        };

        let output = save_template(
            paths,
            file_manipulator,
            on_detect_file_kind,
            &name,
            &source_path,
        )
        .unwrap();

        assert_eq!(expected_error_message, output);
    }
    #[test]
    fn create_template_from_folder() {
        let data_folder = PathBuf::from("user").join("data");
        let name: NotEmptyText = NotEmptyText::new("Some_Name".to_owned()).unwrap();
        let on_detect_file_kind = |_: &_| Ok(FileKind::Folder);
        let source_path = PathBuf::from("some/source");
        let expected_error_message =
            "Created template with name (Some_Name) from the folder \"some/source\"".to_string();

        let paths =
            TestPathProvider::new(data_folder.clone(), PathBuf::from("a"), PathBuf::from("a"));
        let expected_ensured_template_folder = paths.specific_entry_template_files(&name).unwrap();
        let expected_target_folder = expected_ensured_template_folder.clone();

        let file_manipulator = {
            let mut file_manipulator = MockFileManipulator::default();
            file_manipulator
                .expect_ensure_dir()
                .times(1)
                .withf(move |param| param == expected_ensured_template_folder)
                .returning(|_| Ok(()));

            let expected_source_path = source_path.clone();
            file_manipulator
                .expect_copy_dir()
                .times(1)
                .withf(move |actual_source_path, actual_target_folder| {
                    actual_source_path == expected_source_path
                        && actual_target_folder == expected_target_folder
                })
                .returning(|_, _| Ok(()));

            file_manipulator
        };

        let output = save_template(
            paths,
            file_manipulator,
            on_detect_file_kind,
            &name,
            &source_path,
        )
        .unwrap();

        assert_eq!(expected_error_message, output);
    }
    #[test]
    fn may_not_manipulate_files_if_no_detect_file_kind() {
        let name: NotEmptyText = NotEmptyText::new("Some_Name".to_owned()).unwrap();
        let on_detect_file_kind = |_: &_| Err(anyhow!(""));
        let source_path = PathBuf::from("some/source");

        let path_fetcher = MockPathProvider::default();

        let file_manipulator = MockFileManipulator::default();

        save_template(
            path_fetcher,
            file_manipulator,
            on_detect_file_kind,
            &name,
            &source_path,
        )
        .unwrap_err();
    }

    #[test]
    fn load_err_bail_for_source_path_not_querable() {
        let data_folder = PathBuf::from("/some/data");
        let paths = TestPathProvider::new(
            data_folder.clone(),
            data_folder.clone(),
            data_folder.clone(),
        );
        let name = NotEmptyText::new("I am not ther".to_string()).unwrap();
        let assumed_template_entry_path = paths.specific_entry_template_files(&name).unwrap();
        let mut files = MockFileManipulator::default();

        files
            .expect_try_exits()
            .times(1)
            .withf(move |to_check| to_check == assumed_template_entry_path)
            .returning(|_| bail!("a"));
        handle_load_template(paths, files, &name).unwrap_err();
    }

    #[test]
    fn load_err_bail_for_source_path_does_not_exits() {
        let data_folder = PathBuf::from("/some/data");
        let paths = TestPathProvider::new(
            data_folder.clone(),
            data_folder.clone(),
            data_folder.clone(),
        );
        let name = NotEmptyText::new("I am not ther".to_string()).unwrap();
        let mut files = MockFileManipulator::default();
        let assumed_template_entry_path = paths.specific_entry_template_files(&name).unwrap();

        files
            .expect_try_exits()
            .times(1)
            .withf(move |to_check| to_check == assumed_template_entry_path)
            .returning(|_| Ok(false));
        handle_load_template(paths, files, &name).unwrap_err();
    }

    #[test]
    fn load_success_copy_template_to_source_path() {
        let data_folder = PathBuf::from("/some/data");
        let cwd = PathBuf::from("/coding/rust");
        let paths = TestPathProvider::new(data_folder.clone(), data_folder.clone(), cwd.clone());
        let name = NotEmptyText::new("AAA".to_string()).unwrap();
        let assumed_template_entry_path = paths.specific_entry_template_files(&name).unwrap();
        let expected_ensured_template_folder = assumed_template_entry_path.clone();

        // Mocking
        let mut files = MockFileManipulator::default();
        files
            .expect_try_exits()
            .times(1)
            .withf(move |to_check| to_check == assumed_template_entry_path)
            .returning(|_| Ok(true));
        let expected_target_location = cwd.clone();
        files
            .expect_copy_dir()
            .times(1)
            .withf(move |actual_source_path, actual_target_path| {
                actual_source_path == expected_ensured_template_folder
                    && actual_target_path == expected_target_location
            })
            .returning(|_, _| Ok(()));

        // Act
        let output = handle_load_template(paths, files, &name).unwrap();

        // Assert
        let expected_output = format!(
            "Folder {:?} filled with content from Template ({})",
            cwd, name
        );
        assert_eq!(expected_output, output);
    }

    #[test]
    fn list_should_return_all_templates() {
        let mut paths = MockPathProvider::default();
        let expected_template_entry = PathBuf::from("some/all_templates");
        paths
            .expect_general_template_entry()
            .times(1)
            .returning(|| Ok(PathBuf::from("some/all_templates")));
        let mut files = MockFileManipulator::default();
        let expected_ensure_template_entry = expected_template_entry.clone();
        files
            .expect_ensure_dir()
            .times(1)
            .withf(move |actual_template_entry| {
                actual_template_entry == expected_ensure_template_entry
            })
            .returning(|_| Ok(()));
        files
            .expect_list_first_level_dir()
            .times(1)
            .withf(move |actual_listing_path| actual_listing_path == expected_template_entry)
            .returning(|_| Ok(vec![PathBuf::from("rust"), PathBuf::from("python")]));

        let output = handle_list_template(paths, files).unwrap();
        let expected_ouput = format!("{}\nrust\npython\n", constants::TITLE_LIST_RESULT);
        assert_eq!(expected_ouput, output);
    }
}
