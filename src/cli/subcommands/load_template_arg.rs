use clap::Args;

use crate::cli::init_kind::InitKind;
use crate::prelude::*;

#[derive(Debug, Args)]
pub struct LoadTemplateArg {
    name: ValidTemplateName,
    #[arg(long, short, value_enum, env = "CRUST_BOOT_YOU_INIT_KIND", default_value_t = InitKind::OnlyEmpty)]
    with: InitKind,
}

impl LoadTemplateArg {
    pub fn new(name: ValidTemplateName, with: InitKind) -> Self {
        Self { name, with }
    }

    pub fn with(&self) -> InitKind {
        self.with
    }

    pub fn name(&self) -> &ValidTemplateName {
        &self.name
    }
}
