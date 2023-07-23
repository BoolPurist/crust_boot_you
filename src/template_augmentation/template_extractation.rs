use super::{DefaultExtact, KeyExtact};

#[allow(dead_code)]
pub enum TemplateExtractation<'t> {
    FromConsole {
        key: KeyExtact<'t>,
        default_value: DefaultExtact<'t>,
    },
}
