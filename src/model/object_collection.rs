use std::collections::HashMap;

use crate::errors::object_collection_error::ObjectCollectionError;

use super::{entity::EntityTraits, object::{Object, ObjectId}};

#[derive(Debug)]
pub struct ObjectCollection<E: EntityTraits<A>, A> {
    data: HashMap<String, Object<E, A>>,
}

impl<E: EntityTraits<A> + Clone, A: Clone> ObjectCollection<E, A> {
    pub fn new() -> Self {
        ObjectCollection {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, object: &Object<E, A>) {
        self.data.insert(object.get_object_id().get_id().to_string(), object.clone());
    }

    pub fn get_object_by_id(&self, id: ObjectId) -> Result<&Object<E, A>, ObjectCollectionError> {
        self.data
            .get(id.get_id())
            .ok_or_else(|| ObjectCollectionError::MissingObject(id.get_id().to_string()))
    }

    pub fn get_objects_by_meta_id(&self, meta_id:&ObjectId)->Result<Vec<&Object<E, A>>, ObjectCollectionError> {
        let out=self
        .data
        .iter()
        .filter_map(|(_, v)| {
            if let Some(id) = v.get_meta_id() {
                if id.get_id() == meta_id.get_id() {
                    Some(v)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<&Object<E, A>>>();
    
        match !out.is_empty(){
            true => Ok(out),
            false => Err(ObjectCollectionError::NoMatchingObject(meta_id.get_id().to_string()))
        }
        
    }

    pub fn get_objects_by_name(&self, name:&str)->Result<Vec<&Object<E, A>>, ObjectCollectionError> {
        let out=self
        .data
        .iter()
        .filter_map(|(_,v)|if v.get_name()==name {Some(v)} else {None})
        .collect::<Vec<&Object<E, A>>>();
    
        match !out.is_empty(){
            true => Ok(out),
            false => Err(ObjectCollectionError::NoMatchingObject(name.to_string()))
        }
        
    }

    pub fn update_object(&mut self, object: Object<E, A>) {
        self.data.insert(object.get_object_id().get_id().to_string(), object);
    }

    pub fn remove_object(&mut self, id: &str) -> Result<(), ObjectCollectionError> {
        self.data
            .remove(id)
            .ok_or_else(|| ObjectCollectionError::MissingObject(id.to_string()))
            .and_then(|_| Ok(()))
    }

    pub fn list_objects(&self){
        self
        .data
        .iter()
        .for_each(|(k,v)|{
            println!("ID: {} | name: {}", k, v.get_name())
        })
    }
}
