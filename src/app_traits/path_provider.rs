pub use dev_path_provider::DevPathProvider;
pub use prod_path_provider::ProdPathProvider;
pub use test_path_provider::TestPathProvider;

mod dev_path_provider;
mod prod_path_provider;
mod test_path_provider;

use crate::{prelude::*, ValidTemplateName};

pub fn get_root_dev() -> PathBuf {
    std::env::temp_dir().join(constants::dev::TMP_ROOT)
}

pub fn get_root_default_cwd() -> PathBuf {
    get_root_dev().join(constants::dev::TMP_CWD_FOLDER)
}

pub trait PathProvider {
    fn data(&self) -> PathResult;
    fn config(&self) -> PathResult;
    fn logger_file_location(&self) -> PathResult;
    fn logger_folder_location(&self) -> PathResult;

    fn scripts(&self) -> PathResult {
        let data = self.data()?;
        let path = data.join(constants::SCRIPT_FOLDER_NAME);
        info!("Location to scripts: {:?}", path);
        Ok(path)
    }

    fn general_template_entry(&self) -> PathResult {
        let data = self.data()?;
        let path = data.join(constants::TEMPLATES_FOLDER);
        info!("Location of tempaltes: {:?}", path);
        Ok(path)
    }

    fn specific_entry_template_files(&self, template_name: &ValidTemplateName) -> PathResult {
        let named = self.specific_entry_template(template_name)?;
        let path = named.join(constants::FILES_FOLDER);
        info!("Files of template path: {:?}", path);
        Ok(path)
    }

    fn specific_entry_template(&self, template_name: &ValidTemplateName) -> PathResult {
        let general_template_entry = self.general_template_entry()?;
        let named = general_template_entry.join(template_name.as_ref());
        info!("Location of template, ({}): {:?}", template_name, named);
        Ok(named)
    }

    fn template_meta(&self, template_name: &ValidTemplateName) -> PathResult {
        let specific_template_entry = self.specific_entry_template(template_name)?;
        let path_template_config = specific_template_entry.join(constants::TEMPLATE_META_FILE_NAME);
        info!("Path to template config: {:?}", path_template_config);
        Ok(path_template_config)
    }

    fn dictionary(&self) -> PathResult {
        let config = self.config()?;
        let general_config_path = config.join(constants::DICTIONARY_FILE);
        info!("Location of general config path: {:?}", general_config_path);
        Ok(general_config_path)
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn derive_all_other_paths() {
        let path_provider = TestPathProvider::clone_from("root", "data", "config");
        let template_a = ValidTemplateName::new_clone_panic("A");
        let template_b = ValidTemplateName::new_clone_panic("B");
        let actual = [
            path_provider.config(),
            path_provider.data(),
            path_provider.scripts(),
            path_provider.general_template_entry(),
            path_provider.dictionary(),
            path_provider.specific_entry_template(&template_a),
            path_provider.specific_entry_template_files(&template_a),
            path_provider.template_meta(&template_a),
            path_provider.specific_entry_template(&template_b),
            path_provider.specific_entry_template_files(&template_b),
        ]
        .map(|path| path.unwrap());
        insta::assert_debug_snapshot!(actual);
    }
}
