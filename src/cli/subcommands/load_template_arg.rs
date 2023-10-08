use clap::Args;

use crate::cli::from_cli_path::AbsoluteExistingDirPath;
use crate::cli::init_kind::InitKind;
use crate::cli::LoadCliDetails;
use crate::{prelude::*, ValidPlaceholderBorder};

use super::CreateTemplateArg;

#[derive(Debug, Args, Getters)]
pub struct LoadTemplateArg {
    #[arg(long, short)]
    /// Location where the project is initialized.
    target: Option<AbsoluteExistingDirPath>,
    #[command(flatten)]
    details: LoadCliDetails,
}

impl LoadTemplateArg {
    pub fn new(name: ValidTemplateName, with: InitKind) -> Self {
        let details = LoadCliDetails::new(name, with);
        Self {
            details,
            target: None,
        }
    }

    pub fn activate_ignore_placeholders(mut self) -> Self {
        let new_details = self.details.activate_ignore_placeholders();
        self.details = new_details;
        self
    }
    pub fn new_left_delimiter(mut self, left: ValidPlaceholderBorder) -> Self {
        let new_details = self.details.new_left_delimiter(left);
        self.details = new_details;
        self
    }
    pub fn new_right_delimiter(mut self, right: ValidPlaceholderBorder) -> Self {
        let new_details = self.details.new_right_delimiter(right);
        self.details = new_details;
        self
    }
    pub fn new_default_sep(mut self, sep_default: ValidPlaceholderBorder) -> Self {
        let new_details = self.details.new_default_sep(sep_default);
        self.details = new_details;
        self
    }
}

impl TryFrom<CreateTemplateArg> for LoadTemplateArg {
    type Error = AppError;

    fn try_from(value: CreateTemplateArg) -> Result<Self, Self::Error> {
        let details = value.details;
        let target: AbsoluteExistingDirPath = value.target.try_into()?;
        Ok(Self {
            details,
            target: Some(target),
        })
    }
}
