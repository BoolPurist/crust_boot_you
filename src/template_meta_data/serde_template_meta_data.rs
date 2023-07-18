use std::collections::HashSet;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

use super::{AllTemplateMetaData, TemplateMeta};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AllSerdeTemplateMetaData {
    templates: Vec<TemplateMeta>,
}

impl AllSerdeTemplateMetaData {
    pub fn new(templates: Vec<TemplateMeta>) -> Self {
        Self { templates }
    }
}

impl TryFrom<AllSerdeTemplateMetaData> for AllTemplateMetaData {
    type Error = AppError;
    fn try_from(value: AllSerdeTemplateMetaData) -> Result<Self, Self::Error> {
        let mut hash_set = HashSet::new();

        for try_insert in value.templates.iter() {
            if !hash_set.insert(try_insert) {
                bail!(
                    "A template's name ({}) is more than once listed",
                    try_insert.name()
                )
            }
        }

        Ok(Self {
            templates: value.templates,
        })
    }
}

impl From<AllTemplateMetaData> for AllSerdeTemplateMetaData {
    fn from(value: AllTemplateMetaData) -> Self {
        let seq = value.templates.into_iter().collect();
        Self::new(seq)
    }
}
