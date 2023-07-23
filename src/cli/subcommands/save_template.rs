use crate::{cli::AbsoluteExistingPath, prelude::*};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct SaveTemplateCli {
    name: ValidTemplateName,
    path: AbsoluteExistingPath,
}

impl SaveTemplateCli {
    pub fn new(name: ValidTemplateName, path: AbsoluteExistingPath) -> Self {
        Self { name, path }
    }

    pub fn path(&self) -> &AbsoluteExistingPath {
        &self.path
    }

    pub fn name(&self) -> &ValidTemplateName {
        &self.name
    }
}
