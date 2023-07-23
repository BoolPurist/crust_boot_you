mod common;
use common::prelude::*;
use crust_boot_you::handle_commands;

#[test]
#[named]
fn delete_existing_given_template() {
    let setup = TestSetup::new(actual_expected!());

    let given_name = ValidTemplateName::new("to delete".to_string()).unwrap();

    // Act
    let output = handle_commands::handle_delete_template(
        setup.path_provider(),
        setup.os_mani(),
        &given_name,
    )
    .expect("Should be successful in this test case");

    // Assert
    insta::assert_display_snapshot!(output);

    setup.assert_with_expected();
}
#[test]
#[named]
fn delete_err_on_non_existing_template() {
    let setup = TestSetup::new(actual_expected!());

    let given_name = ValidTemplateName::new("to delete".to_string()).unwrap();

    // Act
    let output = handle_commands::handle_delete_template(
        setup.path_provider(),
        setup.os_mani(),
        &given_name,
    )
    .expect_err("Should be an error in this test case");
    setup.assert_with_expected();
    insta::assert_display_snapshot!(output);
}
