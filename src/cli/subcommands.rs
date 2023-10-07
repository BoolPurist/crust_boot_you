use clap::Subcommand;

mod create_template_arg;
mod load_template_arg;
mod save_template;

use crate::ValidTemplateName;

pub use create_template_arg::CreateTemplateArg;
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
    #[clap(alias = "c")]
    Create(CreateTemplateArg),
}
