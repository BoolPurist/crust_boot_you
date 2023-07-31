use crate::prelude::*;

#[derive(Debug, Default)]
pub struct ProdPathProvider;

impl PathProvider for ProdPathProvider {
    fn data(&self) -> PathResult {
        let data = dirs::data_dir().ok_or(anyhow!("Could not retrieve data folder of user"))?;
        let path = data.join(constants::APP_NAME);
        info!("Data path: {:?}", path);
        Ok(path)
    }

    fn config(&self) -> PathResult {
        let data = dirs::config_dir().ok_or(anyhow!("Could not retrieve data folder of user"))?;
        let path = data.join(constants::APP_NAME);
        info!("Config path: {:?}", path);
        Ok(path)
    }

    fn state_dir(&self) -> PathResult {
        let dir = dirs::state_dir()
            .ok_or(anyhow!("Could not get access to state folder on linux."))?
            .join(constants::APP_NAME);
        Ok(dir)
    }

    fn logger_folder_location(&self) -> PathResult {
        let target = if cfg!(target_os = "linux") {
            match self.state_dir() {
                Ok(path) => Ok(path),
                Err(_) => self.data(),
            }
        } else {
            self.data()
        }?;
        let data = target.join(constants::LOG_FOLDER_NAME);
        info!("Path log folder: {:?}", data);
        Ok(data)
    }
}
