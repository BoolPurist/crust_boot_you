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

    fn logger_folder_location(&self) -> PathResult {
        let data = self.data()?.join(constants::LOG_FOLDER_NAME);
        info!("Path log folder: {:?}", data);
        Ok(data)
    }
}
