mod common;
use common::insta_utils;
use common::prelude::*;
use crust_boot_you::cli::{AbsoluteExistingPath, SaveTemplateCli};
use crust_boot_you::handle_commands;

#[test]
#[named]
fn save_file_new_template() {
    let setup = TestSetup::new(actual_expected!());

    let given_name = ValidTemplateName::new("a".to_string()).unwrap();
    let given_path =
        AbsoluteExistingPath::new(PathBuf::from("new.txt"), setup.path_resolver()).unwrap();
    let arguments = SaveTemplateCli::new(given_name, given_path);
    let output =
        handle_commands::handle_save_template(setup.path_provider(), setup.os_mani(), &arguments)
            .expect("Should be successful in this test case");
    insta::with_settings!({ filters => insta_utils::filter_random_tmp_folder_name() }, { insta::assert_display_snapshot!(output) });

    setup.assert_with_expected();
}

#[test]
#[named]
fn save_folder_as_template() {
    let setup = TestSetup::new(actual_expected!());

    let given_name = ValidTemplateName::new("some hello project".to_string()).unwrap();
    let given_path =
        AbsoluteExistingPath::new(PathBuf::from("lang").join("go"), setup.path_resolver()).unwrap();
    let arguments = SaveTemplateCli::new(given_name, given_path);
    let output =
        handle_commands::handle_save_template(setup.path_provider(), setup.os_mani(), &arguments)
            .expect("Should be successful in this test case");

    setup.assert_with_expected();

    insta::with_settings!({ filters => insta_utils::filter_random_tmp_folder_name() }, { insta::assert_display_snapshot!(output) });
}

#[test]
#[named]
fn save_template_err_source_path_no_existence() {
    let setup = TestSetup::new(actual_expected!());

    let _ = AbsoluteExistingPath::new(PathBuf::from("lang").join("go"), setup.path_resolver())
        .expect_err("Should be an error in this test case");

    setup.assert_with_expected();
}

#[test]
#[named]
fn save_template_err_template_already_there() {
    let setup = TestSetup::new(actual_expected!());

    let given_name = ValidTemplateName::new("already_there".to_string()).unwrap();
    let given_path =
        AbsoluteExistingPath::new(PathBuf::from("project"), setup.path_resolver()).unwrap();
    let arguments = SaveTemplateCli::new(given_name, given_path);
    let output =
        handle_commands::handle_save_template(setup.path_provider(), setup.os_mani(), &arguments)
            .expect_err("Should be an error in this test case");
    setup.assert_with_expected();
    insta::assert_display_snapshot!(output);
}
