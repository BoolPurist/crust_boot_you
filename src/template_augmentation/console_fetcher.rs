use super::{template_extractation::ExtractForConsole, OptAugmentationResult};

mod test_console_fetcher;
pub use test_console_fetcher::TestConsoleFetcher;

mod io_console_fetcher;
pub use io_console_fetcher::IoConsoleFetcher;

pub trait ConsoleFetcher {
    fn fetch_from(&self, extract: &ExtractForConsole) -> OptAugmentationResult;
}
