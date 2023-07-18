use clap::Parser;

use super::TemplateCliArg;

#[derive(Debug, Parser)]
pub struct SaveTemplateCli {
    #[command(flatten)]
    arguments: TemplateCliArg,
}

impl SaveTemplateCli {
    pub fn arguments(&self) -> &TemplateCliArg {
        &self.arguments
    }
}
