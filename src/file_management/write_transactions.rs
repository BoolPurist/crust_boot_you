use std::path::PathBuf;

use super::{FileKind, SourceTargetNode};

#[cfg(test)]
use crate::prelude::testing::*;

#[derive(Debug, Into)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct WriteTransactions {
    files: Vec<FileToLoad>,
    folders: Vec<DirToEnsure>,
}

impl WriteTransactions {
    pub fn new(files_dirs: impl IntoIterator<Item = SourceTargetNode>) -> Self {
        let files_dirs = files_dirs.into_iter();
        let cap = files_dirs.size_hint().1.unwrap_or(0);
        let mut files = Vec::with_capacity(cap);
        let mut folders = Vec::with_capacity(cap);

        for next_file_or_dir in files_dirs {
            match next_file_or_dir.node_type() {
                FileKind::File => files.push(FileToLoad(next_file_or_dir.into_target_path())),
                FileKind::Folder => folders.push(DirToEnsure(next_file_or_dir.into_target_path())),
                _ => unreachable!(),
            }
        }

        Self { files, folders }
    }
}

#[derive(Debug, AsRef)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct DirToEnsure(PathBuf);

#[derive(Debug, AsRef)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct FileToLoad(PathBuf);

#[cfg(test)]
mod testing {
    use crate::file_management::NodeEntryMeta;

    use super::*;

    #[test]
    fn separate_files_and_dirs() {
        let (source, target) = (PathBuf::from("/a/source"), PathBuf::from("/a/target"));
        let nodes = [
            NodeEntryMeta::new(FileKind::File, PathBuf::from("/a/source/ccc")),
            NodeEntryMeta::new(FileKind::File, PathBuf::from("/a/source/aaaa/aa")),
            NodeEntryMeta::new(FileKind::Folder, PathBuf::from("/a/source/aaaa")),
            NodeEntryMeta::new(FileKind::Folder, PathBuf::from("/a/source/aaaa/aaa")),
            NodeEntryMeta::new(FileKind::File, PathBuf::from("/a/source/zzz/z")),
            NodeEntryMeta::new(FileKind::Folder, PathBuf::from("/a/source/b")),
        ];
        let input = SourceTargetNode::opt_many_from_many_sources(&source, &target, nodes).unwrap();

        // Act
        let actual = WriteTransactions::new(input);

        insta::assert_ron_snapshot!(actual);
    }
}
