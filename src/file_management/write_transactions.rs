use std::path::PathBuf;

use derive_getters::Getters;
use derive_new::new;

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
                FileKind::File => files.push(next_file_or_dir.into()),
                FileKind::Folder => folders.push(next_file_or_dir.into()),
                _ => unreachable!(),
            }
        }

        Self { files, folders }
    }
}

#[derive(Debug, Getters, Into, new)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct DirToEnsure {
    source: PathBuf,
    target: PathBuf,
}

#[derive(Debug, Getters, Into, new)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct FileToLoad {
    source: PathBuf,
    target: PathBuf,
}

impl From<SourceTargetNode> for FileToLoad {
    fn from(value: SourceTargetNode) -> Self {
        let (source, target) = value.into();
        FileToLoad::new(source, target)
    }
}
impl From<SourceTargetNode> for DirToEnsure {
    fn from(value: SourceTargetNode) -> Self {
        let (source, target) = value.into();
        DirToEnsure::new(source, target)
    }
}

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
