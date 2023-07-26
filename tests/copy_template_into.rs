mod common;
use common::prelude::*;
use crust_boot_you::{
    cli::{InitKind, LoadTemplateArg},
    handle_commands,
    template_augmentation::{console_fetcher::TestConsoleFetcher, RegexTemplateAugmentor},
};
use map_macro::hash_map;

#[test]
#[named]
fn copy_at_target() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd").join("some_where"))
        .build();

    let to_copy_from = ValidTemplateName::new("to_copy".to_string()).unwrap();
    let init_kind = InitKind::OnlyEmpty;
    let arg = LoadTemplateArg::new(to_copy_from, init_kind);
    let mut store: RegexTemplateAugmentor<TestConsoleFetcher> =
        RegexTemplateAugmentor::from_fake(Default::default());
    let output = handle_commands::handle_load_template(
        setup.path_provider(),
        setup.os_mani(),
        &mut store,
        &arg,
    )
    .unwrap();
    setup.assert_with_expected();

    insta_display_filter_random_tmp!(output);
}
#[test]
#[named]
fn copy_err_cwd_not_empty() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd").join("not_empty"))
        .build();

    let to_copy_from = ValidTemplateName::new("a".to_string()).unwrap();
    let init_kind = InitKind::OnlyEmpty;
    let arg = LoadTemplateArg::new(to_copy_from, init_kind);
    let mut store: RegexTemplateAugmentor<TestConsoleFetcher> =
        RegexTemplateAugmentor::from_fake(Default::default());
    let output = handle_commands::handle_load_template(
        setup.path_provider(),
        setup.os_mani(),
        &mut store,
        &arg,
    )
    .unwrap_err();

    setup.assert_with_expected();
    insta_display_filter_random_tmp!(output);
}

#[test]
#[named]
fn copy_err_template_not_found() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd").join("not_empty"))
        .build();

    let to_copy_from = ValidTemplateName::new("not_there".to_string()).unwrap();
    let init_kind = InitKind::OnlyEmpty;
    let arg = LoadTemplateArg::new(to_copy_from, init_kind);
    let mut store: RegexTemplateAugmentor<TestConsoleFetcher> =
        RegexTemplateAugmentor::from_fake(Default::default());
    let output = handle_commands::handle_load_template(
        setup.path_provider(),
        setup.os_mani(),
        &mut store,
        &arg,
    )
    .unwrap_err();

    setup.assert_with_expected();
    insta_display_filter_random_tmp!(output);
}

#[test]
#[named]
fn initializ_with_no_name_conflicts() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd").join("without_name_conflicts"))
        .build();

    let to_copy_from = ValidTemplateName::new("with_placeholders".to_string()).unwrap();
    let init_kind = InitKind::NoNameConflicts;
    let arg = LoadTemplateArg::new(to_copy_from, init_kind);
    let values = hash_map! {"user_name".to_string() => "pattern".to_string()};

    let mut store: RegexTemplateAugmentor<TestConsoleFetcher> =
        RegexTemplateAugmentor::from_fake(values);
    let output = handle_commands::handle_load_template(
        setup.path_provider(),
        setup.os_mani(),
        &mut store,
        &arg,
    )
    .unwrap();

    setup.assert_with_expected();
    insta_display_filter_random_tmp!(output);
}
