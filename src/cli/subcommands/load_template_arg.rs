use clap::Args;

use crate::cli::init_kind::{valid_values_init_kind_msg, InitKind};
use crate::prelude::*;

use const_format::formatcp;
#[derive(Debug, Args)]
pub struct LoadTemplateArg {
    name: NotEmptyText,
    #[arg(long, short, env = "CRUST_BOOT_YOU_INIT_KIND", default_value_t = InitKind::OnlyEmpty, help = help_text_for_init_kind())]
    with: InitKind,
}

const fn help_text_for_init_kind() -> &'static str {
    formatcp!(
        "{}.
        {}: Only copies If there are no previous files/folder inside the target folder. 
        {}: Only copies If there are only previous files/folder 
            which do not share a name of the files/folders from template's content.
        {}: Previous files/folders are overriden 
            if they have a name of File/Folder from content of template.
        {}: All previous files/folders inside target folder are deleted 
            before content of template is copied into target.
",
        valid_values_init_kind_msg(),
        constants::INIT_KIND_ONLY_EMPTY,
        constants::INIT_KIND_NO_NAME_CONFLICT,
        constants::INIT_KIND_OVERRIDE,
        constants::INIT_KIND_PURGE,
    )
}

impl LoadTemplateArg {
    pub fn new(name: NotEmptyText, with: InitKind) -> Self {
        Self { name, with }
    }

    pub fn with(&self) -> InitKind {
        self.with
    }

    pub fn name(&self) -> &NotEmptyText {
        &self.name
    }
}
