use clap::Args;
use derive_getters::Getters;

use crate::cli::{LoadCliDetails, NonFilePath};

#[derive(Debug, Args, Getters, Clone)]
pub struct CreateTemplateArg {
    #[arg(long, short)]
    pub(super) target: NonFilePath,
    #[command(flatten)]
    pub(super) details: LoadCliDetails,
}
