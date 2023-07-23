mod common;
use common::prelude::*;
use crust_boot_you::{
    cli::{InitKind, LoadTemplateArg},
    handle_commands,
};

#[test]
#[named]
fn copy_at_target() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd").join("some_where"))
        .build();

    let to_copy_from = ValidTemplateName::new("to_copy".to_string()).unwrap();
    let init_kind = InitKind::OnlyEmpty;
    let arg = LoadTemplateArg::new(to_copy_from, init_kind);
    let output =
        handle_commands::handle_load_template(setup.path_provider(), setup.os_mani(), &arg)
            .unwrap();
    setup.assert_with_expected();

    insta_display_filter_random_tmp!(output);
}
