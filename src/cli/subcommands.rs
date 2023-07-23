use clap::Subcommand;

mod load_template_arg;
mod save_template;

use crate::ValidTemplateName;
pub use load_template_arg::LoadTemplateArg;
pub use save_template::SaveTemplateCli;

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
