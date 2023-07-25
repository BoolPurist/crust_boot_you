use derive_getters::Getters;
pub use subcommands::SubCommands;

mod cli_loglevel;
mod init_kind;
mod subcommands;

pub use cli_loglevel::CliLogLevel;
pub use from_cli_path::AbsoluteExistingPath;
pub use init_kind::InitKind;
pub use subcommands::LoadTemplateArg;
pub use subcommands::SaveTemplateCli;

mod from_cli_path;

use clap::Parser;

#[derive(Debug, Parser, Getters)]
pub struct AppCliEntry {
    #[clap(short, long, env = "CRUST_BOOT_YOU_RUN_DRY")]
    dry: bool,
    #[clap(short, long, env = "CRUST_BOOT_YOU_TERM_LOGGER")]
    term_logging: bool,
    #[clap(short, long, value_enum, env = "CRUST_BOOT_YOU_LOG_LEVEL")]
    log_level: Option<CliLogLevel>,
    #[clap(short, long, num_args = 1.., value_delimiter = ',', env = "CRUST_BOOT_YOU_LOG_MODULE_FILTER")]
    module_filter: Option<Vec<String>>,
    #[clap(subcommand)]
    sub_commands: SubCommands,
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        AppCliEntry::command().debug_assert()
    }
}
