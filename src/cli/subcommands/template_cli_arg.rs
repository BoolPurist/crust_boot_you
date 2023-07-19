use clap::Args;

use crate::{cli::AbsoluteExistingPath, NotEmptyText};

use super::{check_if_exits, validate_not_empty};

#[derive(Debug, Args)]
pub struct TemplateCliArg {
    #[arg(value_parser = validate_not_empty)]
    name: NotEmptyText,
    #[arg(value_parser = check_if_exits)]
    path: AbsoluteExistingPath,
}

impl TemplateCliArg {
    pub fn path(&self) -> &AbsoluteExistingPath {
        &self.path
    }

    pub fn name(&self) -> &NotEmptyText {
        &self.name
    }
}
