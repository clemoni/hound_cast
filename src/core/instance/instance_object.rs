use std::{collections::HashMap, marker::PhantomData};

use crate::core::{errors::{AttributeError, UniqueIdError}, meta::{meta_entity::MetaEntity, meta_object::MetaObject}, model::{entity::Entity, object::Object, unique_id::Identifier, UniqueId}};

use super::instance_entities::{InstanceAttributes, InstanceEntity};

use crate::core::model::entity::EntityTraits;

pub type InstanceObject = Object<InstanceEntity, InstanceAttributes>;

impl InstanceObject {
    /// Creates a new instance object, converting a meta object ID to an instance object ID.
    pub fn new_instance(
        name: &str,
        entities: HashMap<String, Entity<InstanceAttributes>>,
        meta_id: &UniqueId,
    ) -> Result<Self,UniqueIdError> {
    
        let unique_id_res=UniqueId::new("instance", None);
        match unique_id_res{
            Ok(unique_id) => Ok(Object {
                name: name.to_string(),
                entities,
                id: unique_id,
                meta_id: Some(meta_id.clone()),
                _marker: PhantomData,
            }),
            Err(err) => Err(err),
        }
    }
}

pub struct InstanceObjectBuilder {
    name: String,
    meta_entities: HashMap<String, MetaEntity>,
    instance_entities: HashMap<String, InstanceEntity>,
    meta_id: UniqueId,
}

impl InstanceObjectBuilder {
    /// Creates a new builder for an instance object.
    pub fn new(object: &MetaObject, name: &str) -> Self {
        let object_cloned = object.clone();
        let object_cloned_id=object_cloned.get_id().clone();
        InstanceObjectBuilder {
            name: name.to_string(),
            meta_entities: object_cloned.entities,
            instance_entities: HashMap::new(),
            meta_id: object_cloned_id,
        }
    }

    /// Updates an instance entity with a parsed value from a meta entity.
    pub fn update_entity(
        &mut self,
        entity_name: &str,
        input: Option<&str>,
    ) -> Result<(), AttributeError> {

        let metat_entity = &self
            .meta_entities
            .get(entity_name)
            .ok_or_else(|| AttributeError::NonMatchingType("tt".to_string()))?;

        let instance = metat_entity.get_attribute().parse_attribute(input)?;

        self.instance_entities.insert(
            entity_name.to_string(),
            InstanceEntity::new(entity_name, instance),
        );

        Ok(())
    }

    /// Populates missing meta-entities in the instance object with default values.
    pub fn populate_missing_meta_entites(&mut self) {
        let _ = &self.meta_entities.iter().for_each(|(k, v)| {
            if let None = &self.instance_entities.get(k.as_str()) {
                let attribute = v.get_attribute().insert_none_for_type();
                let name = k.clone();
                self.instance_entities
                    .insert(k.to_string(), InstanceEntity::new(&name, attribute));
            }
        });
    }

    /// Builds and returns the final `InstanceObject`.
    pub fn build(self) -> Result<InstanceObject, UniqueIdError> {
        InstanceObject::new_instance(
            &self.name,
            self.instance_entities,
            &self.meta_id,
        )
    }
}


#[cfg(test)]
mod test {
   
    use crate::core::meta::meta_entity::MetaAttributes;
    use crate::core::model::unique_id::Identifier;
    use super::*;

    #[test]
    fn test_instance_object_creation() {
        let mut meta_object = MetaObject::new_meta("TestMeta").unwrap();
        meta_object.update_entity("attribute1", MetaAttributes::Text);
        meta_object.update_entity("attribute2", MetaAttributes::I16);

        let mut instance_builder = InstanceObjectBuilder::new(&meta_object, "TestInstance");

        instance_builder
            .update_entity("attribute1", Some("value1"))
            .unwrap();
        instance_builder
            .update_entity("attribute2", Some("123"))
            .unwrap();
        instance_builder.populate_missing_meta_entites();

        let instance_object = instance_builder.build().unwrap();

        assert_eq!(instance_object.name, "TestInstance");
        assert!(instance_object.id.get_id().to_string().starts_with("instance"));
        assert_eq!(instance_object.get_meta_id().as_ref().unwrap().get_id(), meta_object.get_id().get_id());
    }

    #[test]
    fn test_update_entity_failed() {
        let meta_object = MetaObject::new_meta("TestMeta").unwrap();

        let mut instance_builder = InstanceObjectBuilder::new(&meta_object, "TestInstance");

        let output = instance_builder.update_entity("attribute1", Some("test_value"));

        assert_eq!(
            output,
            Err(AttributeError::NonMatchingType("tt".to_string()))
        )
    }

    #[test]
    fn test_missing_meta_entities_population() {
        let mut meta_object = MetaObject::new_meta("TestMeta").unwrap();
        meta_object.update_entity("mandatory", MetaAttributes::Text);

        let mut instance_builder = InstanceObjectBuilder::new(&meta_object, "TestInstance");
        instance_builder.populate_missing_meta_entites();

        let instance_object = instance_builder.build().unwrap();
        assert!(instance_object.entities.contains_key("mandatory"));
        assert_eq!(
            instance_object
                .entities
                .get("mandatory")
                .unwrap()
                .get_attribute(),
            &InstanceAttributes::Text(None)
        );
    }

    #[test]
    fn test_build_instance_object() {
        let mut meta_object = MetaObject::new_meta("TestMeta").unwrap();
        meta_object.update_entity("attr1", MetaAttributes::Text);
        meta_object.update_entity("attr2", MetaAttributes::I16);

        let mut instance_builder = InstanceObjectBuilder::new(&meta_object, "TestInstance");
        instance_builder
            .update_entity("attr1", Some("example_value"))
            .unwrap();
        instance_builder.update_entity("attr2", Some("56")).unwrap();

        let instance_object = instance_builder.build().unwrap();
        assert_eq!(instance_object.name, "TestInstance");
        assert!(instance_object.id.get_id().to_string().starts_with("instance"));
    }

    #[test]
    fn test_invalid_update_entity() {
        let mut meta_object = MetaObject::new_meta("TestMeta").unwrap();
        meta_object.update_entity("attr1", MetaAttributes::Text);

        let mut instance_builder = InstanceObjectBuilder::new(&meta_object, "TestInstance");
        let result = instance_builder.update_entity("attr1", Some("wrong_format"));

        assert!(result.is_ok());

        let invalid_result = instance_builder.update_entity("attr2", Some("wrong_type"));
        assert!(invalid_result.is_err());
    }
}
