use std::collections::HashMap;

use crate::core::{instance::instance_entities::{InstanceAttributes, InstanceEntity}, meta::meta_entity::{MetaAttributes, MetaEntity}, model::{entity::EntityTraits, Object, UniqueId}};

use super::errors::CollectionError;

use crate::core::model::unique_id::Identifier;


pub struct ObjectCollection<E: EntityTraits<A>,A>{
    data:HashMap<UniqueId, Object<E,A>>
}

impl <E:EntityTraits<A>+Clone,A:Clone> ObjectCollection<E,A>{
    pub fn new()->ObjectCollection<E,A>{
        ObjectCollection { data: HashMap::new() }
    }

   
    pub fn get(&self, id:&UniqueId)->Result<&Object<E,A>,CollectionError> {
        self.data
        .get(&id.clone())
        .ok_or_else(|| CollectionError::MissingObject(id.get_id().to_string()))
    }

    
    pub fn insert(&mut self, value:&Object<E,A>) {
        self.data.insert(value.get_id().clone(), value.clone());
    }

   
    pub fn remove(&mut self, id:&UniqueId)->Result<(),CollectionError> {
        self.data
        .remove(&id.get_id())
        .ok_or_else(|| CollectionError::MissingObject(id.to_string()))
        .and_then(|_| Ok(()))
    }

    pub fn get_objects_by_meta_id(&self, meta_id:&UniqueId)->Result<Vec<&Object<E, A>>, CollectionError> {
        let output=self
        .data
        .iter()
        .filter_map(|(_, v)| {
            if let Some(id) = v.get_meta_id() {
                if id == meta_id {
                    Some(v)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<&Object<E, A>>>();
    
        if output.is_empty(){
            Err(CollectionError::NoMatchingObject(meta_id.get_id().to_string()))
        }else{
            Ok(output)
        }
        
    }

    pub fn get_objects_by_name(&self, name:&str)->Result<Vec<&Object<E, A>>, CollectionError> {
        let output=self
        .data
        .iter()
        .filter_map(|(_,v)|if v.get_name()==name {Some(v)} else {None})
        .collect::<Vec<&Object<E, A>>>();
    
        if output.is_empty(){
            Err(CollectionError::NoMatchingObject(name.to_string()))
        }else{
            Ok(output)
        }
        
    }

}

pub type MetaObjectCollection=ObjectCollection<MetaEntity,MetaAttributes>;
pub type InstanceObjectCollection=ObjectCollection<InstanceEntity,InstanceAttributes>;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::instance::instance_object::InstanceObject;
    use crate::core::meta::meta_object::MetaObject;
    use crate::core::model::unique_id::Identifier;
    use crate::core::meta::meta_entity::MetaAttributes;

    

    #[test]
    fn test_insert_and_get_object() {
        let mut collection = ObjectCollection::<MetaEntity, MetaAttributes>::new();

        let obj = MetaObject::new_meta("TestObject").unwrap();

        let obj_id=obj.get_id();
    
        collection.insert(&obj);

        let retrieved_obj = collection.get(&obj_id);

        assert!(retrieved_obj.is_ok());
        assert_eq!(retrieved_obj.unwrap().get_name(), "TestObject");
    }

    #[test]
    fn test_remove_object() {
        let mut collection = ObjectCollection::<MetaEntity, MetaAttributes>::new();

        let obj = MetaObject::new_meta("TestObject").unwrap();

        let obj_id=obj.get_id();

        collection.insert(&obj);

        // Remove the object from the collection
        let remove_result = collection.remove(&obj_id);

        assert!(remove_result.is_ok());

        // Verify that the object is removed
        let retrieved_obj = collection.get(&obj_id);
        assert!(retrieved_obj.is_err());
    }

    #[test]
    fn test_get_objects_by_meta_id() {

        let mut meta_collection = MetaObjectCollection::new();
        let mut instance_collection=InstanceObjectCollection::new();

        let meta_obj_1 = MetaObject::new_meta("TestObject1").unwrap();
        let meta_obj_1_id=meta_obj_1.get_id();

        meta_collection.insert(&meta_obj_1);

        let instance_obj_1=InstanceObject::new_instance("Intance1TestParentObject1", HashMap::new(), meta_obj_1_id).unwrap();

        instance_collection.insert(&instance_obj_1);

        let result = instance_collection.get_objects_by_meta_id(&meta_obj_1_id);

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_objects_by_name() {
        let mut collection = ObjectCollection::<MetaEntity, MetaAttributes>::new();

        let obj = MetaObject::new_meta("TestObject").unwrap();

            collection.insert(&obj);

        // Retrieve objects by name
        let result = collection.get_objects_by_name("TestObject");

        assert!(result.is_ok());
        
        assert_eq!(result.as_ref().unwrap().len(), 1);
        assert_eq!(result.unwrap()[0].get_name(), "TestObject");
    }

    #[test]
    fn test_no_matching_object() {
        let collection = ObjectCollection::<MetaEntity, MetaAttributes>::new();

        // Search for a non-existent object
        let result = collection.get_objects_by_name("NonExistentObject");

        assert!(result.is_err());
    }
}