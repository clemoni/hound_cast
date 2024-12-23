//! # TemplateBuilder
//!
//! The `TemplateBuilder` struct provides a builder pattern for constructing `Template` instances.
//! It validates that all required entities in the template match the associated `MetaObject`.

use crate::core::{errors::TemplateError, meta::meta_object::MetaObject, model::unique_id::Identifier};
use super::{template::Template, template_content::TemplateContent};


/// Builder for creating and validating `Template` instances.
pub struct TemplateBuilder<'a> {
    content: String,
    meta_object: &'a MetaObject,
}

impl<'a> TemplateBuilder<'a> {
    /// Instantiates a new `TemplateBuilder`.
    pub fn instanciate(content: &str, meta_object: &'a MetaObject) -> Self {
        TemplateBuilder {
            content: content.to_string(),
            meta_object,
        }
    }

    /// Builds and validates a `Template`.
    pub fn build(self) -> Result<Template, TemplateError> {
        let template_content = TemplateContent::new(&self.content);
        template_content.is_matching_entity(self.meta_object)?;
        Template::new(template_content, self.meta_object.get_id())
        .map_err(|err|TemplateError::PropagatedError(err.to_string()))
    }
}