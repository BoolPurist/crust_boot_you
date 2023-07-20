//! TODO: Remove if used
#![allow(dead_code)]

use std::path::PathBuf;

use super::loaded_node::LoadedNode;
pub struct SourceTargetNode {
    source: LoadedNode,
    target: PathBuf,
}
