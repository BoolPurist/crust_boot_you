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
        io::stdout().flush().map_err(|_| {
            AugmentationError::StdInProblem("Could not flush stdin after printing key as label")
        })?;
        let mut output = String::new();
        std::io::stdin()
            .read_line(&mut output)
            .map_err(|_| AugmentationError::StdInProblem("Could not read line from stdin"))?;
        let without_traling_noise = output.trim_end();
        let to_return = if without_traling_noise.trim_start().is_empty() {
            None
        } else {
            Some(without_traling_noise.to_string())
        };
        println!("");
        Ok(to_return)
    }
}
