use clap::Parser;
use cli::TaskCliEntry;

mod cli;
mod commands;
mod logging;

pub type AppResult<T = ()> = Result<T, anyhow::Error>;
fn main() -> AppResult {
    let cli = TaskCliEntry::parse();
    logging::init_logger();
    let output = commands::handle_commands(cli)?;
    println!("{}", output);
    Ok(())
}
