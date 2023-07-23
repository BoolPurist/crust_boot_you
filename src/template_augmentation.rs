use self::augmentation_error::AugmentationError;

mod augement_repository;
mod console_fetcher;
mod template_extractation;

pub use augement_repository::AugementRepository;
mod augmentation_error;
mod template_replacer;

pub type AugmentationResult<'a> = Result<&'a str, AugmentationError>;
pub type AugementKey = String;

pub type OptAugmentationResult = Result<Option<AugmentValue>, AugmentationError>;

pub type AugmentValue = String;
pub type KeyExtact<'a> = &'a str;
pub type DefaultExtact<'a> = Option<&'a str>;
