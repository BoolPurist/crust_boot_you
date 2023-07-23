use std::borrow::Cow;

use self::augmentation_error::AugmentationError;

mod augment_repository;
mod console_fetcher;
mod template_extractation;

pub use augment_repository::AugementRepository;
pub use regex_replacer::RegexTemplateAugmentor;
mod augmentation_error;
mod regex_replacer;

pub type AugmentationResult<'a> = Result<&'a str, AugmentationError>;
pub type AugementKey = String;

pub type OptAugmentationResult = Result<Option<AugmentValue>, AugmentationError>;

pub type AugmentValue = String;
pub type KeyExtact<'a> = &'a str;
pub type DefaultExtact<'a> = Option<&'a str>;

/// TODO: Provide otther implementation which also keeps track of lines and colums for better error
/// message to end user.
/// The only current implementation regex is limented in that regard.
pub trait TemplateAugmentor {
    fn try_replace<'a>(&mut self, input: &'a str) -> Result<Cow<'a, str>, AugmentationError>;
}
