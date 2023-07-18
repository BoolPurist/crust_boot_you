pub use subcommands::SubCommands;

mod save_template;
mod subcommands;
mod template_cli_arg;

pub use save_template::SaveTemplateCli;
pub use template_cli_arg::TemplateCliArg;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct AppCliEntry {
    #[clap(subcommand)]
    sub_commands: SubCommands,
}

impl AppCliEntry {
    pub fn sub_commands(&self) -> &SubCommands {
        &self.sub_commands
    }
}
