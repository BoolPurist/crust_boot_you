use clap::Parser;

use super::TemplateCliArg;

#[derive(Debug, Parser)]
pub struct SaveTemplateCli {
    #[command(flatten)]
    arguments: TemplateCliArg,
}
impl From<TemplateCliArg> for SaveTemplateCli {
    fn from(value: TemplateCliArg) -> Self {
        Self { arguments: value }
    }
}

impl SaveTemplateCli {
    pub fn arguments(&self) -> &TemplateCliArg {
        &self.arguments
    }
}
