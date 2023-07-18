use crate::{template_meta_data::TemplateMeta, AbsoluteExistingPath};

#[derive(Debug)]
pub struct Template {
    path: AbsoluteExistingPath,
    meta: TemplateMeta,
}

impl Template {
    pub fn new(path: AbsoluteExistingPath, meta: TemplateMeta) -> Self {
        Self { path, meta }
    }
}
