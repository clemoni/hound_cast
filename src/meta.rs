//! The `meta` module defines the `MetaAttributes` enum and related types and functions.
//!
//! `MetaAttributes` describe the types of attributes that can be assigned to entities in a meta-object.
//! These are used as blueprints for the instance objects to ensure type safety and proper initialization.

use crate::{entity::Entity, errors::AttributeError, instance::InstanceAttributes, object::Object};

pub type MetaEntity = Entity<MetaAttributes>;


#[derive(Debug, Clone)]
// Enum representing various types of meta attributes.
/// 
/// These define the data types for attributes, such as `Text` or `I16` (16-bit integer),
/// that can be later instantiated with actual values in instance objects.
pub enum MetaAttributes {
    Text,
    I16,
}

impl MetaAttributes {
   
    /// Parses a raw string input into the corresponding `InstanceAttributes` type.
    /// 
    /// # Arguments
    /// 
    /// - `input`: The optional raw string input to be parsed.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing either the parsed `InstanceAttributes` or an `AttributeError`.
    pub fn parse_attribute(&self, input:Option<&str>)->Result<InstanceAttributes,AttributeError>{
        match &self{
            MetaAttributes::Text => InstanceAttributes::parse_text(input),
            MetaAttributes::I16 =>InstanceAttributes::parse_i16(input),
        }

    }

    /// Provides a default `InstanceAttributes` value for the given `MetaAttributes` type.
    /// 
    /// # Returns
    /// 
    /// An `InstanceAttributes` value with a `None` value appropriate for the type.
    pub fn insert_none_for_type(&self)->InstanceAttributes{
        match &self{
            MetaAttributes::Text => InstanceAttributes::Text(None),
            MetaAttributes::I16 => InstanceAttributes::I16(None),
        }
    }
}

pub type MetaObject = Object<MetaEntity, MetaAttributes>;

impl MetaObject {
    pub fn new_meta(name: &str) -> Self {
        Object::new(name, "meta", None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instance::InstanceAttributes;

    #[test]
    fn test_parse_attribute_text_success() {
        let meta_attr = MetaAttributes::Text;
        let result = meta_attr.parse_attribute(Some("example text"));
        assert_eq!(result, Ok(InstanceAttributes::Text(Some("example text".to_string()))));
    }

    #[test]
    fn test_parse_attribute_text_none() {
        let meta_attr = MetaAttributes::Text;
        let result = meta_attr.parse_attribute(None);
        assert_eq!(result, Ok(InstanceAttributes::Text(None)));
    }

    #[test]
    fn test_parse_attribute_i16_success() {
        let meta_attr = MetaAttributes::I16;
        let result = meta_attr.parse_attribute(Some("123"));
        assert_eq!(result, Ok(InstanceAttributes::I16(Some(123))));
    }

    #[test]
    fn test_parse_attribute_i16_invalid() {
        let meta_attr = MetaAttributes::I16;
        let result = meta_attr.parse_attribute(Some("not a number"));
        assert_eq!(
            result,
            Err(AttributeError::InvalidType("Expect i16 got not a number".to_string()))
        );
    }

    #[test]
    fn test_insert_none_for_type_text() {
        let meta_attr = MetaAttributes::Text;
        let result = meta_attr.insert_none_for_type();
        assert_eq!(result, InstanceAttributes::Text(None));
    }

    #[test]
    fn test_insert_none_for_type_i16() {
        let meta_attr = MetaAttributes::I16;
        let result = meta_attr.insert_none_for_type();
        assert_eq!(result, InstanceAttributes::I16(None));
    }

    #[test]
    fn test_new_meta_object_creation() {
        let meta_object = MetaObject::new_meta("TestMetaObject");

        assert_eq!(meta_object.name, "TestMetaObject");
        assert!(meta_object.entities.is_empty());
        assert!(meta_object.id.starts_with("meta_"));
    }

    // #[test]
    // fn test_update_meta_entity() {
    //     let mut meta_object = MetaObject::new_meta("TestMetaObject");
    //     meta_object.update_entity("attribute1", MetaAttributes::Text);

    //     assert!(meta_object.entities.contains_key("attribute1"));
    //     let entity = meta_object.entities.get("attribute1").unwrap();
    //     assert_eq!(entity.get_name(), "attribute1");
    //     assert_eq!(entity.get_attribute(), &MetaAttributes::Text);
    // }

    #[test]
    fn test_update_multiple_meta_entities() {
        let mut meta_object = MetaObject::new_meta("TestMetaObject");
        meta_object.update_entity("attribute1", MetaAttributes::Text);
        meta_object.update_entity("attribute2", MetaAttributes::I16);

        assert_eq!(meta_object.entities.len(), 2);
        assert!(meta_object.entities.contains_key("attribute1"));
        assert!(meta_object.entities.contains_key("attribute2"));
    }
}