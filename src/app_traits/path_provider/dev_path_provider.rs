use std::path::PathBuf;

use crate::prelude::*;

use super::{PathProvider, PathResult};

#[derive(Debug, Default)]
pub struct DevPathProvider;

impl DevPathProvider {
    fn get_root() -> PathBuf {
        constants::PROJECT_ROOT_PATH.join(constants::dev::ENTRY_FOLDER)
    }
}
impl PathProvider for DevPathProvider {
    fn data(&self) -> PathResult {
        let data = Self::get_root().join(constants::dev::DATA_FOLDER);
        debug!("Data folder {:?}", data);
        Ok(data)
    }

    fn config(&self) -> PathResult {
        let config = Self::get_root().join(constants::dev::CONFIG_FOLDER);
        debug!("Config folder {:?}", config);
        Ok(config)
    }

    fn cwd(&self) -> PathResult {
        let cwd = std::env::current_dir().context("could not access current cwd")?;
        debug!("Cwd: {:?}", &cwd);
        Ok(cwd)
    }
}
