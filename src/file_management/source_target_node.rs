//! TODO: Remove if used
#![allow(dead_code)]

use crate::prelude::*;

use super::{FileKind, NodeEntryMeta};
use std::path::PathBuf;

#[derive(Debug)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct SourceTargetNode {
    source_node: NodeEntryMeta,
    target: PathBuf,
}

impl SourceTargetNode {
    fn new(source_node: NodeEntryMeta, target: PathBuf) -> Self {
        Self {
            source_node,
            target,
        }
    }

    pub fn source_path(&self) -> &Path {
        self.source_node.source_path()
    }
    pub fn target_path(&self) -> &Path {
        self.target.as_path()
    }
    pub fn node_type(&self) -> FileKind {
        *self.source_node.node_type()
    }

    pub fn opt_many_from_many_sources(
        prefix_source: &Path,
        prefix_target: &Path,
        source_to_map: impl IntoIterator<Item = NodeEntryMeta>,
    ) -> Option<Vec<Self>> {
        source_to_map
            .into_iter()
            .map(|node| {
                let target_path = node
                    .source_path()
                    .strip_prefix(prefix_source)
                    .ok()
                    .map(|to_join_with| prefix_target.join(to_join_with))?;
                Some(Self::new(node, target_path))
            })
            .collect()
    }
}

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    fn test_some_if_valid_suffix_and_prefix() {
        let source = Path::new("/some/source");
        let target = Path::new("/some/target");

        let to_map: Vec<NodeEntryMeta> = from_ron_input_file!("source_paths_with_same_prefix.ron");

        // Act
        let actual = SourceTargetNode::opt_many_from_many_sources(source, target, to_map).unwrap();

        insta::assert_ron_snapshot!(actual);
    }
    #[test]
    fn test_none_if_one_file_with_wrong_prefix() {
        use crate::prelude::testing::*;
        let source = Path::new("/some/source");
        let target = Path::new("/some/target");

        let to_map: Vec<NodeEntryMeta> = from_ron_input_file!("different_prefixes.ron");

        // Act
        let actual = SourceTargetNode::opt_many_from_many_sources(source, target, to_map);

        assert!(actual.is_none());
    }
}
