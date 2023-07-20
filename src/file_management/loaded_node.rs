use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LoadedNode {
    Folder { path: PathBuf },
    File { path: PathBuf, content: String },
}

impl LoadedNode {
    pub fn prepend_root(self, root: &Path) -> Self {
        return match self {
            Self::File { path, content } => Self::File {
                path: create_new_p(root, &path),
                content,
            },
            Self::Folder { path } => Self::Folder {
                path: create_new_p(root, &path),
            },
        };

        fn create_new_p(left: &Path, right: &Path) -> PathBuf {
            left.join(right)
        }
    }
}
