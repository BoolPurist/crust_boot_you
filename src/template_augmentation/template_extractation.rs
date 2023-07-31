use super::{DefaultExtact, KeyExtact};

#[derive(Debug)]
pub enum TemplateExtractation<'t> {
    FromConsole(ExtractForConsole<'t>),
}

#[derive(Debug)]
pub struct ExtractForConsole<'a> {
    pub key: KeyExtact<'a>,
    pub default_value: DefaultExtact<'a>,
}

impl<'a> ExtractForConsole<'a> {
    pub fn new(key: KeyExtact<'a>, default_value: DefaultExtact<'a>) -> Self {
        Self { key, default_value }
    }
}
