use clap::Parser;
mod subcommands;
pub use subcommands::TaskSubcommand;

#[derive(Debug, Parser)]
pub struct TaskCliEntry {
    #[clap(subcommand)]
    subcommands: TaskSubcommand,
}

impl TaskCliEntry {
    pub fn subcommands(&self) -> &TaskSubcommand {
        &self.subcommands
    }
}
