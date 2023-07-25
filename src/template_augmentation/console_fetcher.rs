use super::{KeyExtact, OptAugmentationResult};

mod test_console_fetcher;
pub use test_console_fetcher::TestConsoleFetcher;

mod console_fetcher;
pub use console_fetcher::IoConsoleFetcher;

pub trait ConsoleFetcher {
    fn fetch_from(&self, key: KeyExtact<'_>) -> OptAugmentationResult;
}
