use flexi_logger::Logger;

use crate::AppResult;

pub fn init() -> AppResult {
    Logger::try_with_env()?
        .format_for_stderr(flexi_logger::colored_detailed_format)
        .start()?;
    Ok(())
}
