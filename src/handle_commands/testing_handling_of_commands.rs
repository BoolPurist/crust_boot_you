use super::*;
use pretty_assertions::assert_eq;
use std::{io, path::PathBuf};

use crate::app_traits::path_provider::{MockPathProvider, TestPathProvider};
use crate::{app_traits::file_manipulator::MockFileManipulator, cli::InitKind};

fn setup_save_until_template_check(
    path_provider: &mut MockPathProvider,
    file_manipulator: &mut MockFileManipulator,
    name: &NotEmptyText,
    template_path: &Path,
    exits: bool,
) -> PathBuf {
    let expected_name = name.clone();
    let expected_name_for_files = name.clone();
    let expected_template_path = template_path.to_path_buf();
    let expected_template_path_for_files =
        template_path.to_path_buf().join(constants::FILES_FOLDER);

    let return_value = expected_template_path_for_files.to_path_buf();
    let expected_path_to_check = expected_template_path.clone();
    path_provider
        .expect_specific_entry_template()
        .times(1)
        .withf(move |actual_name| *actual_name == expected_name)
        .return_once(|_| Ok(expected_template_path));
    path_provider
        .expect_specific_entry_template_files()
        .times(1)
        .withf(move |actual_name| *actual_name == expected_name_for_files)
        .return_once(|_| Ok(expected_template_path_for_files));

    file_manipulator
        .expect_try_exits()
        .withf(move |actual_path_to_check| actual_path_to_check == expected_path_to_check)
        .return_once(move |_| Ok(exits));

    return_value
}

fn setup_ensure_dir(file_manipulator: &mut MockFileManipulator, expected_target_file_path: &Path) {
    let expected = expected_target_file_path.to_path_buf();
    file_manipulator
        .expect_ensure_dir()
        .times(1)
        .withf(move |param| param == expected)
        .return_once(|_| Ok(()));
}

#[ignore]
#[test]
fn deny_creation_of_template_if_already_there() {
    let mut path_provider = MockPathProvider::default();
    let mut file_manipulator = MockFileManipulator::default();
    let on_file_detect = |_: &_| {
        panic!("Should not come this far");
    };
    let given_name = NotEmptyText::new("Not there".to_string()).unwrap();
    let given_path = PathBuf::from("some_template_name");

    let expected_name = given_name.clone();
    let expected_template_path = PathBuf::from("some/template_path").join(expected_name.as_ref());

    setup_save_until_template_check(
        &mut path_provider,
        &mut file_manipulator,
        &given_name,
        &expected_template_path,
        true,
    );

    let message = save_template(
        path_provider,
        file_manipulator,
        on_file_detect,
        &given_name,
        given_path,
    )
    .unwrap_err();
    let expected_msg = save_err_already_created_template(&given_name);

    assert_eq!(expected_msg, message.to_string());
}

#[ignore]
#[test]
fn create_template_from_one_file() {
    let mut path_provider = MockPathProvider::default();
    let mut file_manipulator = MockFileManipulator::default();
    let on_file_detect = |_: &_| Ok(FileKind::File);
    let given_name = NotEmptyText::new("Not there".to_string()).unwrap();
    let given_path = PathBuf::from("some_template_name");

    let expected_template_path = setup_save_until_template_check(
        &mut path_provider,
        &mut file_manipulator,
        &given_name,
        &given_path,
        false,
    );

    let expected_target_file_path = expected_template_path
        .join(constants::FILES_FOLDER)
        .join(given_name.as_ref());
    setup_ensure_dir(&mut file_manipulator, &expected_template_path);

    let expected_source_path = given_path.clone();
    file_manipulator
        .expect_copy_file()
        .withf(move |actual_source_path, actual_target_path| {
            actual_source_path == expected_source_path
                && actual_target_path == expected_target_file_path
        })
        .returning(|_, _| Ok(()));

    let output = save_template(
        path_provider,
        file_manipulator,
        on_file_detect,
        &given_name,
        &given_path,
    )
    .unwrap();

    let expected_msg = success_save_msg(&given_name, "file", &given_path);
    assert_eq!(expected_msg, output);
}
#[test]
fn create_template_from_folder() {
    let mut path_provider = MockPathProvider::default();
    let mut file_manipulator = MockFileManipulator::default();
    let on_file_detect = |_: &_| Ok(FileKind::Folder);
    let given_name = NotEmptyText::new("I am a folder/ hey".to_string()).unwrap();
    let given_path = PathBuf::from("some_template_name");

    let expected_template_path = setup_save_until_template_check(
        &mut path_provider,
        &mut file_manipulator,
        &given_name,
        &given_path,
        false,
    );

    setup_ensure_dir(&mut file_manipulator, &expected_template_path);

    let expected_source_path = given_path.clone();
    let expected_target_path = expected_template_path.clone();
    file_manipulator
        .expect_copy_dir()
        .times(1)
        .withf(move |actual_source_path, actual_target_path| {
            actual_source_path == expected_source_path && actual_target_path == expected_target_path
        })
        .return_once(|_, _| Ok(()));

    let output = save_template(
        path_provider,
        file_manipulator,
        on_file_detect,
        &given_name,
        &given_path,
    )
    .unwrap();

    let expected_error_message = success_save_msg(&given_name, "folder", &given_path);
    assert_eq!(expected_error_message, output);
}
#[test]
fn may_not_manipulate_files_if_no_folder_or_file() {
    let name: NotEmptyText = NotEmptyText::new("Some_Name".to_owned()).unwrap();
    let on_detect_file_kind = |_: &_| Ok(FileKind::Symlink);
    let source_path = PathBuf::from("some/source");
    let mut path_fetcher = MockPathProvider::default();
    let mut file_manipulator = MockFileManipulator::default();

    setup_save_until_template_check(
        &mut path_fetcher,
        &mut file_manipulator,
        &name,
        &source_path,
        false,
    );

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
        .returning(|_| Err(AppIoError::custom("a")));
    let args = LoadTemplateArg::new(name, InitKind::default());
    handle_load_template(paths, files, &args).unwrap_err();
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
    let args = LoadTemplateArg::new(name, InitKind::default());
    handle_load_template(paths, files, &args).unwrap_err();
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
    let args = LoadTemplateArg::new(name.clone(), InitKind::default());
    let output = handle_load_template(paths, files, &args).unwrap();

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
        .withf(move |actual_template_entry| actual_template_entry == expected_ensure_template_entry)
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

fn setup_act_assert_delete(
    expected_delete_return: AppIoResult,
    given_name: NotEmptyText,
) -> ReturnToUser {
    let mut path_provider = MockPathProvider::default();

    let mut file_manipulator = MockFileManipulator::default();

    let expected_template_path = PathBuf::from("/some/data").join(given_name.as_ref());
    let expected_target_path = expected_template_path.clone();
    let expected_name = given_name.clone();
    path_provider
        .expect_specific_entry_template()
        .times(1)
        .withf(move |actual_name| *actual_name == expected_name)
        .return_once(|_| Ok(expected_template_path));
    file_manipulator
        .expect_delete_whole_folder()
        .withf(move |actual_target_folder| actual_target_folder == expected_target_path)
        .return_once(move |_| expected_delete_return);
    handle_delete_template(path_provider, file_manipulator, &given_name)
}

#[test]
fn delete_template() {
    let given_name = NotEmptyText::new("to_delete".to_string()).unwrap();
    let expected_message = format!("Template ({}) was deleted.", given_name.as_ref());
    let message = setup_act_assert_delete(Ok(()), given_name).unwrap();
    assert_eq!(expected_message, message);
}
#[test]
fn delete_err_if_no_template() {
    let given_name = NotEmptyText::new("not_there".to_string()).unwrap();
    let expected_message = error_delete_msg_not_found(&given_name);
    let message = setup_act_assert_delete(Err(AppIoError::NotFound), given_name).unwrap_err();
    assert_eq!(expected_message, message.to_string());
}
#[test]
fn delete_some_other_err() {
    let given_name = NotEmptyText::new("not_there".to_string()).unwrap();

    let error = io::Error::new(io::ErrorKind::Other, "some error").into();
    let expected_message = error_delet_msg_other_err(&given_name, error);
    let message = setup_act_assert_delete(
        Err(io::Error::new(io::ErrorKind::Other, "some error").into()),
        given_name,
    )
    .unwrap_err();
    assert_eq!(expected_message, message.to_string());
}
