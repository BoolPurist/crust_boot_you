use clap::Args;

use crate::{cli::AbsoluteExistingPath, ValidTemplateName};

use super::check_if_exits;

#[derive(Debug, Args)]
pub struct TemplateCliArg {
    name: ValidTemplateName,
    #[arg(value_parser = check_if_exits)]
    path: AbsoluteExistingPath,
}

impl TemplateCliArg {
    pub fn path(&self) -> &AbsoluteExistingPath {
        &self.path
    }

    pub fn name(&self) -> &ValidTemplateName {
        &self.name
    }
}
