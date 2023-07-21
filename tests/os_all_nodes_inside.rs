mod common;
use common::prelude::*;
use crust_boot_you::file_management::{FileKind, NodeEntryMeta};

#[test]
#[named]
fn list_all_nodes_from_files() {
    let setup = TestSetup::new(actual_expected!());
    let file_mani = setup.os_mani();
    let tmp_path = setup.path_to_temp();
    let output: Vec<NodeEntryMeta> = file_mani.all_nodes_inside(tmp_path).unwrap();
    let striped = strip_away_prefix_from_meta(&tmp_path, output);

    insta::assert_debug_snapshot!(striped);

    setup.assert_with_expected();
}

#[test]
#[named]
fn list_all_nodes_from_dirs_and_files() {
    let setup = TestSetup::new(actual_expected!());
    let file_mani = setup.os_mani();
    let tmp_path = setup.path_to_temp();
    let output: Vec<NodeEntryMeta> = file_mani.all_nodes_inside(tmp_path).unwrap();
    let striped = strip_away_prefix_from_meta(&tmp_path, output);

    insta::assert_debug_snapshot!(striped);
    setup.assert_with_expected();
}
fn strip_away_prefix_from_meta(
    prefix: &Path,
    to_strip: impl IntoIterator<Item = NodeEntryMeta>,
) -> Vec<NodeEntryMeta> {
    let (unstriped_paths, types): (Vec<PathBuf>, Vec<FileKind>) =
        to_strip.into_iter().map(From::from).unzip();

    common::strip_away_changing_temp_prefix(prefix, &unstriped_paths)
        .into_iter()
        .zip(types)
        .map(|(striped, node_type)| NodeEntryMeta::new(node_type, striped))
        .collect()
}
