mod common;
use common::prelude::*;

#[test]
#[named]
fn list_all_nodes_1_level_only_without_files() {
    let setup = TestSetup::new(actual_expected!());
    let file_mani = setup.os_mani();
    let tmp_path = setup.path_to_temp();
    let actual = file_mani.list_first_level_dir(&tmp_path).unwrap();
    let actual_stripped = common::strip_away_changing_temp_prefix(&tmp_path, actual.as_slice());
    insta::assert_debug_snapshot!(actual_stripped);

    setup.assert_with_expected();
}
#[test]
#[named]
fn list_nothing_for_there_no_folders() {
    let setup = TestSetup::new(actual_expected!());
    let file_mani = setup.os_mani();
    let tmp_path = setup.path_to_temp();
    let actual = file_mani.list_first_level_dir(&tmp_path).unwrap();
    let actual_stripped = common::strip_away_changing_temp_prefix(&tmp_path, actual.as_slice());
    insta::assert_debug_snapshot!(actual_stripped);

    setup.assert_with_expected();
}
