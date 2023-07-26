pub type AugmentationResult<'a> = Result<&'a str, AugmentationError>;
pub type AugementKey = String;
pub type OptAugmentationResult = Result<Option<AugmentValue>, AugmentationError>;
pub type AugmentValue = String;
pub type KeyExtact<'a> = &'a str;
pub type DefaultExtact<'a> = Option<&'a str>;

pub type FakeAugmentStore = HashMap<AugementKey, AugmentValue>;
pub type TestAugmentStore = HashMap<AugementKey, AugmentValue>;

pub use augment_repository::AugementRepository;
pub use augmentation_error::AugmentationError;
pub use regex_replacer::RegexTemplateAugmentor;

pub mod console_fetcher;

mod augment_repository;
mod augmentation_error;
mod regex_replacer;
mod template_extractation;

use std::{borrow::Cow, collections::HashMap};

/// TODO: Provide otther implementation which also keeps track of lines and colums for better error
/// message to end user.
/// The only current implementation regex is limented in that regard.
pub trait TemplateAugmentor {
    fn try_replace<'a>(&mut self, input: &'a str) -> Result<Cow<'a, str>, AugmentationError>;
}
