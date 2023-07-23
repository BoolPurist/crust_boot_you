use common::prelude::*;
mod common;

use crust_boot_you::handle_commands;

#[named]
#[test]
fn list_avaiable_templates() {
    let setup = TestSetup::only_actual(actual!());

    let result =
        handle_commands::handle_list_template(setup.path_provider(), setup.os_mani()).unwrap();
    setup.assert_with_expected();

    insta_display_filter_random_tmp!(result);
}

#[named]
#[test]
fn list_template_no_template() {
    let setup = TestSetup::only_actual(actual!());

    let test_paths = TestPathProvider::clone_from(setup.path_to_temp(), "data", "config");

    let result = handle_commands::handle_list_template(&test_paths, setup.os_mani()).unwrap();
    setup.assert_with_expected();
    insta::assert_display_snapshot!(result);
}
