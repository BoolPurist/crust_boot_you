use crate::{prelude::*, NotEmptyText};
mod dev_path_provider;
pub use dev_path_provider::DevPathProvider;

pub trait PathProvider {
    fn data(&self) -> PathResult;
    fn config(&self) -> PathResult;

    fn scripts(&self) -> PathResult {
        let data = self.data()?;
        Ok(data.join(constants::SCRIPT_FOLDER_NAME))
    }

    fn general_template_entry(&self) -> PathResult {
        let data = self.data()?;
        Ok(data.join(constants::TEMPLATES_FOLDER))
    }

    fn cwd(&self) -> PathResult;

    fn specific_entry_template_files(&self, template_name: &NotEmptyText) -> PathResult {
        let general_template_entry = self.general_template_entry()?;
        let named = general_template_entry.join(template_name.as_ref());
        Ok(named.join(constants::FILES_FOLDER))
    }

    fn template_meta(&self, template_name: &NotEmptyText) -> PathResult {
        let specific_template_entry = self.specific_entry_template_files(template_name)?;
        Ok(specific_template_entry.join(constants::TEMPLATE_META_FILE_NAME))
    }

    fn dictionary(&self) -> PathResult {
        let config = self.config()?;
        Ok(config.join(constants::DICTIONARY_FILE))
    }
}
