use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TaskSubcommand {
    Init,
    Reset,
    Clear,
}
