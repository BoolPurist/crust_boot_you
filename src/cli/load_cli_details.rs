use clap::{arg, Args};
use derive_getters::Getters;

use crate::{app_env_name, constants, ValidPlaceholderBorder, ValidTemplateName};

use super::InitKind;

#[derive(Debug, Args, Getters, Clone)]
pub struct LoadCliDetails {
    name: ValidTemplateName,
    #[arg(long, short, value_enum, env = app_env_name!("INIT_KIND"), default_value_t = InitKind::OnlyEmpty)]
    with: InitKind,
    #[arg(long, short, env = app_env_name!("IGNORE_PLACEHOLDERS"))]
    ignore_placeholders: bool,
    #[arg(long, short,default_value_t = constants::DEFAULT_LEFT_DELIMITER.parse().unwrap(),  env = app_env_name!("LEFT_DELIMITER"))]
    left_delimiter: ValidPlaceholderBorder,
    #[arg(long, short, default_value_t = constants::DEFAULT_RIGHT_DELIMITER.parse().unwrap(), env = app_env_name!("RIGHT_DELIMITER"))]
    right_delimiter: ValidPlaceholderBorder,
    #[arg(long, short,
        default_value_t = constants::SEPERATOR_BETWEEN_DEFAULT_AND_VALUE.to_string().parse().unwrap(),
        env = app_env_name!("SEP_VAL_DEFAULT")
    )]
    sep_val_default: ValidPlaceholderBorder,
}

impl LoadCliDetails {
    pub fn new(name: ValidTemplateName, with: InitKind) -> Self {
        Self {
            name,
            with,
            ignore_placeholders: false,
            sep_val_default: ValidPlaceholderBorder::new(
                constants::SEPERATOR_BETWEEN_DEFAULT_AND_VALUE.to_string(),
            )
            .unwrap(),
            left_delimiter: ValidPlaceholderBorder::new(
                constants::DEFAULT_LEFT_DELIMITER.to_owned(),
            )
            .unwrap(),
            right_delimiter: ValidPlaceholderBorder::new(
                constants::DEFAULT_RIGHT_DELIMITER.to_owned(),
            )
            .unwrap(),
        }
    }

    pub fn activate_ignore_placeholders(mut self) -> Self {
        self.ignore_placeholders = true;
        self
    }
    pub fn new_left_delimiter(mut self, left: ValidPlaceholderBorder) -> Self {
        self.left_delimiter = left;
        self
    }
    pub fn new_right_delimiter(mut self, right: ValidPlaceholderBorder) -> Self {
        self.right_delimiter = right;
        self
    }
    pub fn new_default_sep(mut self, sep_default: ValidPlaceholderBorder) -> Self {
        self.sep_val_default = sep_default;
        self
    }
}
