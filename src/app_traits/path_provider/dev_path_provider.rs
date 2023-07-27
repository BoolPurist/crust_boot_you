use crate::prelude::*;

use super::{PathProvider, PathResult};

#[derive(Debug)]
pub struct DevPathProvider {
    root: PathBuf,
}

impl DevPathProvider {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Path {
        self.root.as_path()
    }
}

impl Default for DevPathProvider {
    fn default() -> Self {
        let root = super::get_root_dev();
        Self { root: root.clone() }
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

    fn logger_file_location(&self) -> PathResult {
        let file_path = self
            .logger_folder_location()?
            .join(constants::dev::LOGGER_DEV_FILE);
        info!("Log folder path: {:?}", file_path);
        Ok(file_path)
    }

    fn logger_folder_location(&self) -> PathResult {
        let folder_log = PathBuf::from(constants::project_root());
        info!("Log folder path: {:?}", &folder_log);
        Ok(folder_log)
    }
}
