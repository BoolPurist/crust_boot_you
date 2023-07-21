use common::named;
use crust_boot_you::{file_management::FileNodeMeta, prelude::*};

use crate::common::TestSetup;
mod common;

#[test]
#[named]
fn list_all_nodes_from_files() {
    let setup = TestSetup::new(actual_expected!());
    let file_mani = setup.os_mani();
    let tmp_path = setup.path_to_temp();
    let output: Vec<FileNodeMeta> = file_mani.all_nodes_inside(tmp_path).unwrap();
    let striped = strip_away_changing_temp_prefix(&tmp_path, &output);

    insta::assert_debug_snapshot!(striped);

    setup.assert_with_expected();
}

#[test]
#[named]
fn list_all_nodes_from_dirs_and_files() {
    let setup = TestSetup::new(actual_expected!());
    let file_mani = setup.os_mani();
    let tmp_path = setup.path_to_temp();
    let output: Vec<FileNodeMeta> = file_mani.all_nodes_inside(tmp_path).unwrap();
    let striped = strip_away_changing_temp_prefix(&tmp_path, &output);

    insta::assert_debug_snapshot!(striped);
    setup.assert_with_expected();
}

fn strip_away_changing_temp_prefix(prefix: &Path, to_strip: &[FileNodeMeta]) -> Vec<FileNodeMeta> {
    to_strip
        .iter()
        .map(|to_strip| {
            let new_path = to_strip
                .source_path()
                .strip_prefix(prefix)
                .unwrap()
                .to_owned();
            FileNodeMeta::new(*to_strip.node_type(), new_path)
        })
        .collect()
}
