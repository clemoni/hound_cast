//! The `instance` module provides `InstanceAttributes` and related functionality.
//!
//! `InstanceAttributes` represent the actual values of attributes for specific entities
//! in an instance object, matching the schema defined by `MetaAttributes`.

use std::{collections::HashMap, marker::PhantomData};

use crate::{entity::{Entity, EntityTraits}, errors::AttributeError, meta::{MetaEntity, MetaObject}, object::Object};

type InstanceEntity = Entity<InstanceAttributes>;

#[derive(Debug, Clone, PartialEq)]
/// Enum representing the actual values for various attribute types in an instance object.
/// 
/// These are initialized based on the meta attributes and populated with real data.
pub enum InstanceAttributes {
    Text(Option<String>),
    I16(Option<i16>),
}


impl InstanceAttributes{

    /// Parses an optional string input into a `Text` attribute.
    pub fn parse_text(input:Option<&str>)->Result<InstanceAttributes, AttributeError>{
        match input{
            Some(txt) => Ok(InstanceAttributes::Text(Some(txt.to_string()))),
            None => Ok(InstanceAttributes::Text(None)),
        }
    }
    
    /// Parses an optional string input into an `I16` attribute.
    pub fn parse_i16(input:Option<&str>)->Result<InstanceAttributes, AttributeError>{
        match input{
            Some(i16) =>{
                i16.parse::<i16>()
                .map(|num|InstanceAttributes::I16(Some(num)))
                .map_err(|_|AttributeError::InvalidType(format!("Expect i16 got {}",i16)))
            },
            None => Ok(InstanceAttributes::I16(None)),
        }
    }
}

type InstanceObject = Object<InstanceEntity, InstanceAttributes>;



impl InstanceObject {
    /// Creates a new instance object, converting a meta object ID to an instance object ID.
    pub fn new_instance(name: &str, id:&str, entities:HashMap<String, Entity<InstanceAttributes>>, meta_name:&str) -> Self {
        Object { 
            name: name.to_string(), 
            entities, 
            id: id.replace("meta_", "instance_"), 
            meta_name: Some(meta_name.to_string()), 
            _marker: PhantomData
        }
    }
    
}


pub struct InstanceObjectBuilder {
    name: String,
    meta_entities: HashMap<String, MetaEntity>,
    instance_entities: HashMap<String, InstanceEntity>,
    meta_name:String,
    meta_id:String,
}

impl InstanceObjectBuilder {
    /// Creates a new builder for an instance object.
    pub fn new(object: MetaObject, name:&str) -> Self {
        let object_cloned = object.clone();
        InstanceObjectBuilder {
            name: name.to_string(),
            meta_entities: object_cloned.entities,
            instance_entities: HashMap::new(),
            meta_name:object_cloned.name.to_string(),
            meta_id:object_cloned.id.to_string()
        }
    }
    
    /// Updates an instance entity with a parsed value from a meta entity.
    pub fn update_entity(&mut self, entity_name: &str, input:Option<&str>)->Result<(), AttributeError>{
       
       let metat_entity=&self.meta_entities.get(entity_name).ok_or_else(||AttributeError::NonMatchingType("tt".to_string()))?;
       
       let instance = metat_entity.get_attribute().parse_attribute(input)?;

       self.instance_entities.insert(entity_name.to_string(), InstanceEntity::new(entity_name, instance));

       Ok(())

    }
    
    /// Populates missing meta-entities in the instance object with default values.
    pub fn populate_missing_meta_entites(&mut self){
        let _=&self.meta_entities
        .iter()
        .for_each(|(k,v)|{
            if let None = &self.instance_entities.get(k.as_str()){
                let attribute=v.get_attribute().insert_none_for_type();
                let name=k.clone();
                self.instance_entities.insert(k.to_string(), InstanceEntity::new(&name, attribute));
            }
        });
    }

    /// Builds and returns the final `InstanceObject`.
    pub fn build(self) -> InstanceObject {
        
       InstanceObject::new_instance(&self.name,&self.meta_id,self.instance_entities, &self.meta_name)
       
    }
}


#[cfg(test)]
mod test{
    use crate::meta::MetaAttributes;

    use super::*;

    #[test]
    fn test_success_parse_text() {
        let output = InstanceAttributes::parse_text(Some("test"));
        assert_eq!(output, Ok(InstanceAttributes::Text(Some("test".to_string()))));
    }

    #[test]
    fn test_success_parse_text_none() {
        let output = InstanceAttributes::parse_text(None);
        assert_eq!(output, Ok(InstanceAttributes::Text(None)));
    }

    #[test]
    fn test_success_parse_i16() {
        let output = InstanceAttributes::parse_i16(Some("33"));
        assert_eq!(output, Ok(InstanceAttributes::I16(Some(33))));
    }

    #[test]
    fn test_success_parse_i16_none() {
        let output = InstanceAttributes::parse_i16(None);
        assert_eq!(output, Ok(InstanceAttributes::I16(None)));
    }

    #[test]
    fn test_success_parse_i16_failed() {
        let output = InstanceAttributes::parse_i16(Some("failed"));
        assert_eq!(output, Err(AttributeError::InvalidType("Expect i16 got failed".to_string())));
    }

    #[test]
    fn test_parse_i16_none() {
        let result = InstanceAttributes::parse_i16(None);
        assert_eq!(result, Ok(InstanceAttributes::I16(None)));
    }

    #[test]
    fn test_instance_object_creation() {
        let mut meta_object = MetaObject::new_meta("TestMeta");
        meta_object.update_entity("attribute1", MetaAttributes::Text);
        meta_object.update_entity("attribute2", MetaAttributes::I16);

        let mut instance_builder = InstanceObjectBuilder::new(meta_object.clone(), "TestInstance");

        instance_builder.update_entity("attribute1", Some("value1")).unwrap();
        instance_builder.update_entity("attribute2", Some("123")).unwrap();
        instance_builder.populate_missing_meta_entites();

        let instance_object = instance_builder.build();

        assert_eq!(instance_object.name, "TestInstance");
        assert!(instance_object.id.starts_with("instance_"));
        assert_eq!(instance_object.meta_name.unwrap(), "TestMeta");
    }

    #[test]
    fn test_update_entity_failed() {
        let meta_object = MetaObject::new_meta("TestMeta");

        let mut instance_builder = InstanceObjectBuilder::new(meta_object, "TestInstance");

       let output= instance_builder.update_entity("attribute1", Some("test_value"));

        assert_eq!(output, Err(AttributeError::NonMatchingType("tt".to_string())))
    }

    #[test]
    fn test_missing_meta_entities_population() {
        let mut meta_object = MetaObject::new_meta("TestMeta");
        meta_object.update_entity("mandatory", MetaAttributes::Text);

        let mut instance_builder = InstanceObjectBuilder::new(meta_object.clone(), "TestInstance");
        instance_builder.populate_missing_meta_entites();

        let instance_object = instance_builder.build();
        assert!(instance_object.entities.contains_key("mandatory"));
        assert_eq!(
            instance_object.entities.get("mandatory").unwrap().get_attribute(),
            &InstanceAttributes::Text(None)
        );
    }

    #[test]
    fn test_build_instance_object() {
        let mut meta_object = MetaObject::new_meta("TestMeta");
        meta_object.update_entity("attr1", MetaAttributes::Text);
        meta_object.update_entity("attr2", MetaAttributes::I16);

        let mut instance_builder = InstanceObjectBuilder::new(meta_object, "TestInstance");
        instance_builder.update_entity("attr1", Some("example_value")).unwrap();
        instance_builder.update_entity("attr2", Some("56")).unwrap();

        let instance_object = instance_builder.build();
        assert_eq!(instance_object.name, "TestInstance");
        assert!(instance_object.id.starts_with("instance_"));
    }

    #[test]
    fn test_invalid_update_entity() {
        let mut meta_object = MetaObject::new_meta("TestMeta");
        meta_object.update_entity("attr1", MetaAttributes::Text);

        let mut instance_builder = InstanceObjectBuilder::new(meta_object, "TestInstance");
        let result = instance_builder.update_entity("attr1", Some("wrong_format"));

        assert!(result.is_ok());

        let invalid_result = instance_builder.update_entity("attr2", Some("wrong_type"));
        assert!(invalid_result.is_err());
    }


    



}


