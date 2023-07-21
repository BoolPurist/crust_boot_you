use std::cell::Cell;

use crate::prelude::*;

use super::{PathProvider, PathResult};

#[derive(Debug)]
pub struct DevPathProvider {
    cwd: PathBuf,
    root: PathBuf,
    has_set_cwd: Cell<bool>,
}

impl DevPathProvider {
    pub fn root(&self) -> &Path {
        self.root.as_path()
    }
}

impl Default for DevPathProvider {
    fn default() -> Self {
        let root = super::get_root_dev();
        Self {
            root: root.clone(),
            cwd: root.join(constants::dev::TMP_CWD_FOLDE),
            has_set_cwd: Cell::new(false),
        }
    }
}

impl PathProvider for DevPathProvider {
    fn data(&self) -> PathResult {
        let data = self.root.join(constants::dev::DATA_FOLDER);
        debug!("Data folder {:?}", data);
        Ok(data)
    }

    fn config(&self) -> PathResult {
        let config = self.root.join(constants::dev::CONFIG_FOLDER);
        debug!("Config folder {:?}", config);
        Ok(config)
    }

    fn cwd(&self) -> PathResult {
        let cwd = &self.cwd;
        if !self.has_set_cwd.get() {
            std::env::set_current_dir(&cwd).unwrap_or_else(|_| {
                panic!("Failed to set CWD.\nCwd at {:?} does not exist.", &cwd)
            });
            self.has_set_cwd.set(true);
        }
        info!("Using some temp folder as cwd !");
        info!("Set Cwd to: {:?}", &cwd);
        Ok(cwd.clone())
    }
}
