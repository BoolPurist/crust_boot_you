use clap::Args;
use derive_getters::Getters;

use crate::cli::{LoadCliDetails, NonFilePath};

#[derive(Debug, Args, Getters, Clone)]
/// Initializes a project at some path. If path does not exit then the folder is automatically
/// created  
pub struct CreateTemplateArg {
    #[arg(long, short)]
    /// Location where the project is initialized.
    pub(super) target: NonFilePath,
    #[command(flatten)]
    pub(super) details: LoadCliDetails,
}
