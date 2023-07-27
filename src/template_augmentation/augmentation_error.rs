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
        let line = find_nearest_line_before_pos(input, place);
        Self { line, error }
    }
}
fn find_nearest_line_before_pos(input: &str, pos: usize) -> usize {
    find_all_new_lines(input)
        .take_while(|&index| index < pos)
        .count()
        + 1
}
fn find_all_new_lines(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .char_indices()
        .flat_map(|(index, symb)| if symb == '\n' { Some(index) } else { None })
}
