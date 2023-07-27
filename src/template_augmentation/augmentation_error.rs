use crate::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Line:{0}: {1}", self.line, self.error)]
pub struct AugmentationError {
    line: usize,
    error: AppError,
}

impl AugmentationError {
    pub fn new(input: &str, place: usize, error: AppError) -> Self {
        let line = find_all_new_lines(input)
            .take_while(|&index| index < place)
            .count()
            + 1;
        Self { line, error }
    }
}
fn find_all_new_lines(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .char_indices()
        .flat_map(|(index, symb)| if symb == '\n' { Some(index) } else { None })
}
