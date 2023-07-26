use super::{DefaultExtact, KeyExtact};

#[derive(Debug)]
pub enum TemplateExtractation<'t> {
    FromConsole {
        key: KeyExtact<'t>,
        default_value: DefaultExtact<'t>,
    },
}
