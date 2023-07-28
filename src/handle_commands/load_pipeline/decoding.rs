use super::{ByteContent, Text};

use crate::{file_management::write_transactions::FileToLoad, prelude::*};

#[derive(Debug)]
pub enum DecodedContent {
    NotDecodedable(ByteContent),
    Decoded(DecodedText),
}

#[derive(Debug, AsRef, Deref)]
pub struct DecodedText(Text);

#[derive(Debug, Getters, Into)]
pub struct DecodedFile {
    source: Box<Path>,
    target: Box<Path>,
    content: DecodedContent,
}

impl DecodedFile {
    pub fn new(source: Box<Path>, target: Box<Path>, content: DecodedContent) -> Self {
        Self {
            source,
            target,
            content,
        }
    }
    pub fn new_content(self, new_text: Box<str>) -> Self {
        Self {
            content: DecodedContent::Decoded(DecodedText(new_text)),
            ..self
        }
    }
}

impl DecodedContent {
    pub fn from_raw(bytes: ByteContent) -> Self {
        match std::str::from_utf8(&bytes) {
            Ok(text) => Self::Decoded(DecodedText(text.into())),
            Err(_) => Self::NotDecodedable(bytes),
        }
    }
    pub fn from_text(text: String) -> Self {
        Self::Decoded(DecodedText(text.into()))
    }

    pub fn to_byte_ref(&self) -> &[u8] {
        match self {
            Self::NotDecodedable(bytes) => bytes,
            Self::Decoded(text) => text.as_bytes(),
        }
    }
}

pub fn decode_files(
    file_access: &impl FileManipulator,
    to_load: Vec<FileToLoad>,
) -> AppResult<Vec<DecodedFile>> {
    to_load
        .into_iter()
        .map(|next| {
            let (source, target) = next.into();
            let raw = file_access.read_bytes(&source)?;
            let decoded = DecodedContent::from_raw(raw.into());
            Ok(DecodedFile::new(source.into(), target.into(), decoded))
        })
        .collect()
}
