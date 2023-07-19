use std::path::PathBuf;

use clap::Subcommand;

use crate::NotEmptyText;

mod save_template;
mod template_cli_arg;
pub use save_template::SaveTemplateCli;
pub use template_cli_arg::TemplateCliArg;

use super::AbsoluteExistingPath;

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    #[clap(alias = "s")]
    SaveTemplate(SaveTemplateCli),
    #[clap(alias = "l")]
    LoadTemplate {
        #[arg(value_parser = validate_not_empty)]
        name: NotEmptyText,
    },
    #[clap(alias = "lt")]
    ListTemplate,
}

fn validate_not_empty(input: &str) -> Result<NotEmptyText, String> {
    NotEmptyText::new(input.to_string()).map_err(|error| error.to_string())
}
fn check_if_exits(input: &str) -> Result<AbsoluteExistingPath, String> {
    let path: PathBuf = input.into();
    AbsoluteExistingPath::new(path).map_err(|error| error.to_string())
}
