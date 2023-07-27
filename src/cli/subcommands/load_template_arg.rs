use clap::Args;

use crate::cli::init_kind::InitKind;
use crate::{prelude::*, app_env_name};

#[derive(Debug, Args, Getters)]
pub struct LoadTemplateArg {
    name: ValidTemplateName,
    #[arg(long, short, value_enum, env = app_env_name!("INIT_KIND"), default_value_t = InitKind::OnlyEmpty)]
    with: InitKind,
    #[arg(long, short, env = app_env_name!("IGNORE_PLACEHOLDERS"))]
    ignore_placeholders: bool,
}

impl LoadTemplateArg {
    pub fn new(name: ValidTemplateName, with: InitKind) -> Self {
        Self {
            name,
            with,
            ignore_placeholders: false,
        }
    }

    pub fn activate_ignore_placeholders(mut self) -> Self {
        self.ignore_placeholders = true;
        self
    }
}
