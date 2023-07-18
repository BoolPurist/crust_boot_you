use crate::{template_meta_data::TemplateMeta, AbsoluteExistingPath};

#[derive(Debug)]
pub struct Template {
    /// Todo inactive
    _path: AbsoluteExistingPath,
    _meta: TemplateMeta,
}

impl Template {
    pub fn new(path: AbsoluteExistingPath, meta: TemplateMeta) -> Self {
        Self {
            _path: path,
            _meta: meta,
        }
    }
}
