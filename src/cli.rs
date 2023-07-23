pub use subcommands::SubCommands;

mod init_kind;
mod subcommands;

pub use from_cli_path::AbsoluteExistingPath;
pub use init_kind::InitKind;
pub use subcommands::LoadTemplateArg;
pub use subcommands::SaveTemplateCli;

mod from_cli_path;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct AppCliEntry {
    #[clap(short, long, env = "RUN_DRY")]
    dry: Option<bool>,
    #[clap(subcommand)]
    sub_commands: SubCommands,
}

impl AppCliEntry {
    pub fn sub_commands(&self) -> &SubCommands {
        &self.sub_commands
    }

    pub fn dry(&self) -> bool {
        if cfg!(debug_assertions) {
            self.dry.unwrap_or(true)
        } else {
            self.dry.unwrap_or(false)
        }
    }
}
