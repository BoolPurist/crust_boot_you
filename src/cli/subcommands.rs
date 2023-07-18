use clap::Subcommand;

use crate::NotEmptyText;

use super::SaveTemplateCli;

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    #[clap(alias = "s")]
    SaveTemplate(SaveTemplateCli),
    #[clap(alias = "l")]
    LoadTemplate {
        #[arg(value_parser = validate_not_empty)]
        name: NotEmptyText,
    },
}

fn validate_not_empty(input: &str) -> Result<NotEmptyText, String> {
    NotEmptyText::new(input.to_string()).map_err(|error| error.to_string())
}
