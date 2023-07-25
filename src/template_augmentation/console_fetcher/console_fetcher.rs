use std::io;

use super::ConsoleFetcher;
use crate::template_augmentation::augmentation_error::AugmentationError;
use std::io::Write;

#[derive(Default, Debug)]
pub struct IoConsoleFetcher;

impl ConsoleFetcher for IoConsoleFetcher {
    fn fetch_from(
        &self,
        key: crate::template_augmentation::KeyExtact<'_>,
    ) -> crate::template_augmentation::OptAugmentationResult {
        print!("{}: ", key);
        io::stdout()
            .flush()
            .map_err(|_| AugmentationError::StdInProblem)?;
        let mut output = String::new();
        std::io::stdin()
            .read_line(&mut output)
            .map_err(|_| AugmentationError::StdInProblem)?;
        let output = if output.trim().is_empty() {
            None
        } else {
            Some(output)
        };
        println!("");
        Ok(output)
    }
}
