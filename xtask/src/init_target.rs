use std::path::PathBuf;

use crust_boot_you::LoadedNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitTarget {
    path: PathBuf,
    nodes: Vec<LoadedNode>,
}

impl InitTarget {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn nodes(&self) -> &Vec<LoadedNode> {
        &self.nodes
    }
}
