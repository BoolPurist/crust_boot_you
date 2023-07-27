mod common;
use common::prelude::*;
use crust_boot_you::{
    cli::{InitKind, LoadTemplateArg},
    handle_commands,
    template_augmentation::{
        console_fetcher::TestConsoleFetcher, AugementRepository, RegexTemplateAugmentor,
    },
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

#[test]
#[named]
fn purge_and_initialze() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd").join("to_delete"))
        .build();

    let to_copy_from = ValidTemplateName::new("with_placeholders".to_string()).unwrap();
    let init_kind = InitKind::Purge;
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

#[test]
#[named]
fn override_it() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd").join("override_it"))
        .build();

    let to_copy_from = ValidTemplateName::new("with_placeholders".to_string()).unwrap();
    let init_kind = InitKind::Override;
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

#[test]
#[named]
fn correct_line_number_error() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd"))
        .build();

    let to_copy_from = ValidTemplateName::new("missing_value".to_string()).unwrap();
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
    .unwrap_err();

    let source = output.source().unwrap().to_string();
    let output = format!("{}\n{}", output, source);

    setup.assert_with_expected();
    insta_display_filter_random_tmp!(output);
}

#[test]
#[named]
fn ignore_templating() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd"))
        .build();

    let to_copy_from = ValidTemplateName::new("ignore_template".to_string()).unwrap();
    let init_kind = InitKind::NoNameConflicts;
    let arg = LoadTemplateArg::new(to_copy_from, init_kind).activate_ignore_placeholders();
    let values = hash_map! {"world".to_string() => "pattern".to_string()};

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
#[test]
#[named]
fn with_different_placeholder() {
    let setup = TestSetupBuilder::new(actual_expected!())
        .suffix_cwd(PathBuf::from("cwd"))
        .build();

    let deliminter = "%%".to_string();
    let to_copy_from = ValidTemplateName::new("!!!".to_string()).unwrap();
    let init_kind = InitKind::Override;

    let arg = LoadTemplateArg::new(to_copy_from, init_kind)
        .new_left_delimiter(deliminter.clone().parse().unwrap())
        .new_right_delimiter(deliminter.parse().unwrap());
    let values = hash_map! {"world".to_string() => "P x!!x C".to_string()};

    let console_fetcher = TestConsoleFetcher::new(values);
    let repo = AugementRepository::new(console_fetcher);

    let mut store: RegexTemplateAugmentor<TestConsoleFetcher> =
        RegexTemplateAugmentor::from_cli(repo, &arg);

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
