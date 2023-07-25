use super::{KeyExtact, OptAugmentationResult};

mod test_console_fetcher;
pub use test_console_fetcher::TestConsoleFetcher;

mod io_console_fetcher;
pub use io_console_fetcher::IoConsoleFetcher;

pub trait ConsoleFetcher {
    fn fetch_from(&self, key: KeyExtact<'_>) -> OptAugmentationResult;
}
