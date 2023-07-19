pub use subcommands::SubCommands;

mod subcommands;

pub use from_cli_path::AbsoluteExistingPath;
pub use subcommands::SaveTemplateCli;
pub use subcommands::TemplateCliArg;

mod from_cli_path;

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
