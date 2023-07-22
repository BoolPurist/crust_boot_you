use std::path::PathBuf;

use clap::Subcommand;

mod load_template_arg;
mod save_template;
mod template_cli_arg;

use crate::ValidTemplateName;

use super::AbsoluteExistingPath;
pub use load_template_arg::LoadTemplateArg;
pub use save_template::SaveTemplateCli;
pub use template_cli_arg::TemplateCliArg;

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    #[clap(alias = "s")]
    SaveTemplate(SaveTemplateCli),
    #[clap(alias = "l")]
    LoadTemplate(LoadTemplateArg),
    #[clap(alias = "tl")]
    ListTemplate,
    #[clap(alias = "d")]
    DeleteTemplate { name: ValidTemplateName },
}

fn check_if_exits(input: &str) -> Result<AbsoluteExistingPath, String> {
    let path: PathBuf = input.into();
    AbsoluteExistingPath::new(path).map_err(|error| error.to_string())
}
