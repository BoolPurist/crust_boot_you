use crate::prelude::*;

pub use serde_template_meta_data::AllSerdeTemplateMetaData;
pub use template::TemplateMeta;

mod serde_template_meta_data;
mod template;

#[derive(Debug, Default, Clone)]
pub struct AllTemplateMetaData {
    /// Keep this data for preserving order when saved.
    templates: Vec<TemplateMeta>,
}

impl AllTemplateMetaData {
    pub fn get_template(&self, name: &str) -> Option<&TemplateMeta> {
        self.templates.iter().find(|meta| meta.name() == name)
    }

    pub fn insert_template(&mut self, new: TemplateMeta) -> AppResult {
        if self.get_template(new.name()).is_none() {
            self.templates.push(new);
            Ok(())
        } else {
            bail!("Name {} already exits", new.name());
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::NotEmptyText;

    #[test]
    fn test_from_app_to_serde_template_meta() {
        let templates = vec![
            TemplateMeta::new(NotEmptyText::new_clone_panic("a")),
            TemplateMeta::new(NotEmptyText::new_clone_panic("b")),
        ];
        let input: AllTemplateMetaData = AllTemplateMetaData { templates };
        let serde_version: AllSerdeTemplateMetaData = input.into();
        insta::assert_debug_snapshot!(serde_version);
    }
    #[test]
    fn test_from_serde_to_template_meta_app() {
        let data = vec![
            TemplateMeta::new(NotEmptyText::new_clone_panic("a")),
            TemplateMeta::new(NotEmptyText::new_clone_panic("b")),
        ];
        let input = AllSerdeTemplateMetaData::new(data);
        let serde_version: AppResult<AllTemplateMetaData> = input.try_into();
        insta::assert_debug_snapshot!(serde_version);
    }
    #[test]
    fn test_detect_duplicate_serde_templat_for_meta_template() {
        let data = vec![
            TemplateMeta::new(NotEmptyText::new_clone_panic("a")),
            TemplateMeta::new(NotEmptyText::new_clone_panic("b")),
            TemplateMeta::new(NotEmptyText::new_clone_panic("a")),
        ];
        let input = AllSerdeTemplateMetaData::new(data);
        let serde_version: AppResult<AllTemplateMetaData> = input.try_into();
        assert!(serde_version.is_err(), "Should have reported as an error");
        insta::assert_debug_snapshot!(serde_version);
    }
}
