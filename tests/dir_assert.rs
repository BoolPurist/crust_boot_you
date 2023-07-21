use crate::common::dir_asserts::DirAssert;
use common::dir_asserts::assert_folders;
use common::prelude::*;
mod common;

#[named]
#[test]
fn assert_if_equals_on_one_file() {
    let (actual, expected) = actual_expected!();
    let output = assert_folders(&actual, &expected).unwrap();
    assert!(DirAssert::Equal == output, "\n{}\n", output);
}

#[named]
#[test]
fn assert_if_detect_more_actual_file() {
    let (actual, expected) = actual_expected!();
    let output = assert_folders(&actual, &expected).unwrap();
    assert!(
        matches!(output, DirAssert::ActualMore(one_more) if one_more == PathBuf::from("one_more.txt")),
        "Actual folder shoud have had one more file"
    );
}

#[named]
#[test]
fn assert_if_detect_more_expected_file() {
    let (actual, expected) = actual_expected!();
    let output = assert_folders(&actual, &expected).unwrap();
    assert!(
        matches!(output, DirAssert::ExpectedMore(one_more) if one_more == PathBuf::from("one_more.txt")),
        "Actual folder shoud have had one more file"
    );
}

#[named]
#[test]
fn assert_if_detect_different_paths() {
    let (actual, expected) = actual_expected!();
    let output = assert_folders(&actual, &expected).unwrap();
    assert!(matches!(output, DirAssert::DifferentPaths(..)));
}

#[named]
#[test]
fn assert_if_detect_different_type() {
    let (actual, expected) = actual_expected!();
    let output = assert_folders(&actual, &expected).unwrap();
    assert!(matches!(output, DirAssert::DifferentFileType(..)));
}

#[named]
#[test]
fn assert_if_detect_different_content() {
    let (actual, expected) = actual_expected!();
    let output = assert_folders(&actual, &expected).unwrap();
    assert!(matches!(output, DirAssert::DifferentContent(..)));
}
