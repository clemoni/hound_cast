//! # Template
//!
//! The `Template` struct is the core of the `Template` module, responsible for storing the template content and
//! a reference to its parent `ObjectId`. It provides methods to populate the template using data from an `InstanceObject`.
//!
//! ## Responsibilities:
//! - Validates the relationship between the template and `InstanceObject`.
//! - Populates placeholders with actual values.

use crate::{
    errors::template_error::TemplateError, instance::instance_object::InstanceObject, meta::meta_object::MetaObject, model::{
        entity::EntityTraits,
        object::ObjectId,
    }
};

use super::template_content::TemplateContent;



/// Represents a template bound to a specific `ObjectId`.
#[derive(Debug)]
pub struct Template<'a> {
    content: TemplateContent,
    parent_object: &'a ObjectId,
}

impl<'a> Template<'a> {
    /// Creates a new `Template`.
    pub fn new(content: TemplateContent, parent_object: &'a ObjectId) -> Self {
        Template {
            content,
            parent_object,
        }
    }

    /// Transforms the content by replacing placeholders with actual values from the `InstanceObject`.
    fn transform_content(content: &str, object: &InstanceObject) -> String {
        content
            .split_whitespace()
            .flat_map(|word| {
                TemplateContent::get_pattern()
                    .captures(TemplateContent::clean_word(word))
                    .and_then(|caps| object.entities.get(&caps["reference"]))
                    .map(|entity| entity.get_attribute().to_string())
                    .or_else(|| Some(word.to_string()))
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Builds the final content by populating the template with values from the `InstanceObject`.
    pub fn build_from_instance(&self, object: &InstanceObject) -> Result<String, TemplateError> {
        match &object.meta_id {
            Some(meta_id) if self.parent_object == meta_id => {
                Ok(Self::transform_content(&self.content.get_content(), object))
            }
            Some(_) => Err(TemplateError::UnauthrorisedActionFromMetaObject {
                meta: self.parent_object.get_id().to_string(), // Assuming `ObjectId` implements `Display`
                instance: object.name.to_string(),
            }),
            None => Err(TemplateError::MissingMetaObjectId(object.name.to_string())),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        instance::instance_object::InstanceObjectBuilder,
        meta::{meta_entity::MetaAttributes, meta_object::MetaObject}, template::template_builder::TemplateBuilder,
    };

    #[test]
    fn valid_template_creation() {
        // Test case for successful template creation and population.
        let mut meta_object = MetaObject::new_meta("TestMeta");
        meta_object.update_entity("attribute1", MetaAttributes::Text);
        meta_object.update_entity("attribute2", MetaAttributes::I16);

        let mut instance_builder = InstanceObjectBuilder::new(&meta_object, "TestInstance");
        instance_builder.update_entity("attribute1", Some("value1")).unwrap();
        instance_builder.update_entity("attribute2", Some("123")).unwrap();
        let instance_object = instance_builder.build();

        let content = "This is a test with [@attribute1] and [@attribute2]";
        let template = TemplateBuilder::instanciate(content, &meta_object).build().unwrap();
        let populated_content = template.build_from_instance(&instance_object).unwrap();

        assert_eq!(populated_content, "This is a test with value1 and 123");
    }

    #[test]
    fn fail_template_creation_non_matching_entities() {
        // Test case for missing entities.
        let mut meta_object = MetaObject::new_meta("TestMeta");
        meta_object.update_entity("attribute1", MetaAttributes::Text);
        meta_object.update_entity("attribute2", MetaAttributes::I16);

        let content = "This is a test with [@attribute1]";
        let template_result = TemplateBuilder::instanciate(content, &meta_object).build();

        assert!(matches!(
            template_result.unwrap_err(),
            TemplateError::MissingEntitiesFromMetaObject(_)
        ));
    }
}