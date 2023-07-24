//! TODO: Remove if used
#![allow(dead_code)]

use crate::prelude::*;
use std::path::PathBuf;

use super::FileKind;

#[derive(Debug, Clone, new, Getters, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Serialize, Deserialize, PartialOrd, Ord))]
pub struct NodeEntryMeta {
    node_type: FileKind,
    source_path: PathBuf,
}

impl From<NodeEntryMeta> for (PathBuf, FileKind) {
    fn from(value: NodeEntryMeta) -> Self {
        (value.source_path, value.node_type)
    }
}

impl<'a> From<&'a NodeEntryMeta> for (&'a Path, FileKind) {
    fn from(value: &'a NodeEntryMeta) -> Self {
        (&value.source_path, value.node_type)
    }
}
