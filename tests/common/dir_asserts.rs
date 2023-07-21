#![allow(dead_code)]
use std::fs::FileType;

use crust_boot_you::prelude::*;

pub type DifferentPaths = (PathBuf, PathBuf);
pub type DifferentType = (FileType, FileType);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DirAssert {
    Equal,
    ActualMore(PathBuf),
    ExpectedMore(PathBuf),
    DifferentPaths(DifferentPaths),
    DifferentFileType(DifferentPaths, DifferentType),
    DifferentContent(DifferentPaths),
}

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use walkdir::WalkDir;

pub fn assert_folders(actual: &Path, expected: &Path) -> AppResult<DirAssert> {
    let (actual_root, exepected_root) = (actual, expected);
    let actual = WalkDir::new(actual).sort_by_file_name();
    let expected = WalkDir::new(expected).sort_by_file_name();
    for actual_expected in actual.into_iter().zip_longest(expected) {
        match actual_expected {
            Both(actual, expected) => {
                let actual = actual?;
                let expected = expected?;
                let (actual_path, expected_path) = (actual.path(), expected.path());
                let (rel_actual_path, rel_expected_path) = (
                    actual_path.strip_prefix(actual_root).unwrap(),
                    expected_path.strip_prefix(exepected_root).unwrap(),
                );
                let (actual_type, expected_type) = (actual.file_type(), expected.file_type());

                if rel_actual_path != rel_expected_path {
                    return Ok(DirAssert::DifferentPaths(paths_as_errors(
                        rel_actual_path,
                        rel_expected_path,
                    )));
                }
                if actual_type != expected_type {
                    let to_return = DirAssert::DifferentFileType(
                        paths_as_errors(rel_actual_path, rel_expected_path),
                        (actual_type, expected_type),
                    );
                    return Ok(to_return);
                }

                if actual_type.is_file() {
                    let (actual_content, expected_content) =
                        (std::fs::read(actual_path)?, std::fs::read(expected_path)?);

                    if actual_content != expected_content {
                        return Ok(DirAssert::DifferentContent(paths_as_errors(
                            rel_actual_path,
                            rel_expected_path,
                        )));
                    }
                }
            }
            Left(actual) => {
                let actual_path = actual?.path().to_path_buf();
                let rel_actual = actual_path.strip_prefix(actual_root).unwrap().to_path_buf();
                return Ok(DirAssert::ActualMore(rel_actual));
            }
            Right(expected) => {
                let expected_path = expected?.path().to_path_buf();
                let rel_expected = expected_path
                    .strip_prefix(exepected_root)
                    .unwrap()
                    .to_path_buf();
                return Ok(DirAssert::ExpectedMore(rel_expected));
            }
        }
    }

    return Ok(DirAssert::Equal);

    fn paths_as_errors(left: &Path, right: &Path) -> DifferentPaths {
        (left.to_path_buf(), right.to_path_buf())
    }
}
