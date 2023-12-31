use crate::{prelude::*, template_augmentation::template_extractation::ExtractForConsole};
use std::io;

use super::ConsoleFetcher;
use std::io::Write;

#[derive(Default, Debug)]
pub struct IoConsoleFetcher;

impl ConsoleFetcher for IoConsoleFetcher {
    fn fetch_from(
        &self,
        extract: &ExtractForConsole<'_>,
    ) -> crate::template_augmentation::OptAugmentationResult {
        let ExtractForConsole { key, default_value } = extract;
        if let Some(hint_to) = default_value {
            print!("{} (Defaults to \"{}\"): ", key, hint_to);
        } else {
            print!("{}: ", key);
        };

        io::stdout()
            .flush()
            .context("Could not flush stdin after printing key as label")?;
        let mut output = String::new();
        std::io::stdin()
            .read_line(&mut output)
            .context("Could not read line from stdin")?;
        let output = output.trim_end();

        let to_return = if output.trim_start().is_empty() {
            None
        } else {
            Some(output.to_string())
        };
        println!();
        Ok(to_return)
    }
}
