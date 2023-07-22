use crate::{prelude::*, ValidTemplateName};
mod dev_path_provider;
pub use dev_path_provider::DevPathProvider;
mod test_path_provider;
pub use test_path_provider::TestPathProvider;

pub fn get_root_dev() -> PathBuf {
    std::env::temp_dir().join(constants::dev::TMP_ROOT)
}

#[cfg_attr(test, automock)]
pub trait PathProvider {
    fn data(&self) -> PathResult;
    fn config(&self) -> PathResult;
    fn cwd(&self) -> PathResult;

    fn scripts(&self) -> PathResult {
        let data = self.data()?;
        Ok(data.join(constants::SCRIPT_FOLDER_NAME))
    }

    fn general_template_entry(&self) -> PathResult {
        let data = self.data()?;
        Ok(data.join(constants::TEMPLATES_FOLDER))
    }

    fn specific_entry_template_files(&self, template_name: &ValidTemplateName) -> PathResult {
        let named = self.specific_entry_template(template_name)?;
        Ok(named.join(constants::FILES_FOLDER))
    }

    fn specific_entry_template(&self, template_name: &ValidTemplateName) -> PathResult {
        let general_template_entry = self.general_template_entry()?;
        let named = general_template_entry.join(template_name.as_ref());
        Ok(named)
    }

    fn template_meta(&self, template_name: &ValidTemplateName) -> PathResult {
        let specific_template_entry = self.specific_entry_template(template_name)?;
        Ok(specific_template_entry.join(constants::TEMPLATE_META_FILE_NAME))
    }

    fn dictionary(&self) -> PathResult {
        let config = self.config()?;
        Ok(config.join(constants::DICTIONARY_FILE))
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn derive_all_other_paths() {
        let path_provider = TestPathProvider::clone_from("root", "data", "config", "cwd");
        let template_a = ValidTemplateName::new_clone_panic("A");
        let template_b = ValidTemplateName::new_clone_panic("B");
        let actual = [
            path_provider.cwd(),
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
