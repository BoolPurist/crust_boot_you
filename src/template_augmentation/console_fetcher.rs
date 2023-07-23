use super::{KeyExtact, OptAugmentationResult};
#[cfg(test)]
mod test_console_fetcher;
#[cfg(test)]
pub use test_console_fetcher::TestConsoleFetcher;

pub trait ConsoleFetcher {
    fn fetch_from(&self, key: KeyExtact<'_>) -> OptAugmentationResult;
}
