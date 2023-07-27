use clap::Args;

use crate::cli::init_kind::InitKind;
use crate::{app_env_name, prelude::*};

#[derive(Debug, Args, Getters)]
pub struct LoadTemplateArg {
    name: ValidTemplateName,
    #[arg(long, short, value_enum, env = app_env_name!("INIT_KIND"), default_value_t = InitKind::OnlyEmpty)]
    with: InitKind,
    #[arg(long, short, env = app_env_name!("IGNORE_PLACEHOLDERS"))]
    ignore_placeholders: bool,
    #[arg(long, short,default_value_t = String::from(constants::DEFAULT_LEFT_DELIMITER),  env = app_env_name!("LEFT_DELIMITER"))]
    left_delimiter: String,
    #[arg(long, short, default_value_t = String::from(constants::DEFAULT_RIGHT_DELIMITER), env = app_env_name!("RIGHT_DELIMITER"))]
    right_delimiter: String,
}

impl LoadTemplateArg {
    pub fn new(name: ValidTemplateName, with: InitKind) -> Self {
        Self {
            name,
            with,
            ignore_placeholders: false,
            left_delimiter: String::from(constants::DEFAULT_LEFT_DELIMITER),
            right_delimiter: String::from(constants::DEFAULT_RIGHT_DELIMITER),
        }
    }

    pub fn activate_ignore_placeholders(mut self) -> Self {
        self.ignore_placeholders = true;
        self
    }
    pub fn new_left_delimiter(mut self, left: String) -> Self {
        self.left_delimiter = left;
        self
    }
    pub fn new_right_delimiter(mut self, right: String) -> Self {
        self.right_delimiter = right;
        self
    }
}
