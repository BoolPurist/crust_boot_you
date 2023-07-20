//! TODO: Remove if used
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use super::FileKind;

#[derive(Debug)]
pub struct FileNodeMeta {
    node_type: FileKind,
    source_path: PathBuf,
}

impl FileNodeMeta {
    pub fn new(node_type: FileKind, source_path: PathBuf) -> Self {
        Self {
            node_type,
            source_path,
        }
    }

    pub fn source_path(&self) -> &Path {
        self.source_path.as_path()
    }

    pub fn node_type(&self) -> &FileKind {
        &self.node_type
    }
}
