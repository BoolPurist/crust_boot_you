use std::path::PathBuf;

use clap::Args;

use crate::{absolue_existing_path::AbsoluteExistingPath, NotEmptyText};

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

fn validate_not_empty(input: &str) -> Result<NotEmptyText, String> {
    NotEmptyText::new(input.to_string()).map_err(|error| error.to_string())
}
fn check_if_exits(input: &str) -> Result<AbsoluteExistingPath, String> {
    let path: PathBuf = input.into();
    AbsoluteExistingPath::new(path).map_err(|error| error.to_string())
}
