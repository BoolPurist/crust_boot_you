use crate::prelude::*;

use super::{PathProvider, PathResult};

#[derive(Debug, Default)]
pub struct DevPathProvider;

impl PathProvider for DevPathProvider {
    fn data(&self) -> PathResult {
        let data = super::get_root_dev().join(constants::dev::DATA_FOLDER);
        debug!("Data folder {:?}", data);
        Ok(data)
    }

    fn config(&self) -> PathResult {
        let config = super::get_root_dev().join(constants::dev::CONFIG_FOLDER);
        debug!("Config folder {:?}", config);
        Ok(config)
    }

    fn cwd(&self) -> PathResult {
        let cwd = super::get_root_dev();
        info!("Using temp folder as cwd !");
        info!("Cwd: {:?}", &cwd);
        Ok(cwd)
    }
}
