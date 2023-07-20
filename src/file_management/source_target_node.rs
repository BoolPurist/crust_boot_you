use std::path::PathBuf;

use super::FileNode;

pub struct SourceTargetNode {
    source: FileNode,
    target: PathBuf,
}
