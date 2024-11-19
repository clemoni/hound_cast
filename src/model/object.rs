//! The `object` module provides the `Object` struct and related implementations.
//!
//! An `Object` represents a collection of entities, either in meta form (defining a schema)
//! or instance form (populated with real values).

use std::{collections::HashMap, marker::PhantomData, process::Output};

use uuid::Uuid;

use super::entity::EntityTraits;

#[derive(Debug, Clone,PartialEq)]
pub struct ObjectId(String);

impl ObjectId{
    pub fn new(prefix:&str)->Self{
        ObjectId(format!("{}_{}",prefix,Uuid::new_v4()))
       
    }

    pub fn get_id(&self)->&str{
        &self.0
    }
}


#[derive(Debug, Clone)]
/// Represents a collection of entities.
///
/// This can be either a `MetaObject` defining the schema or an `InstanceObject` containing real data.
pub struct Object<E: EntityTraits<A>, A> {
    pub name: String,
    pub entities: HashMap<String, E>,
    pub id: ObjectId,
    pub meta_id: Option<ObjectId>,
    pub _marker: PhantomData<A>,
}

impl<E: EntityTraits<A> + Clone, A: Clone> Object<E, A> {
    /// Creates a new object with the given name and ID prefix.
    pub fn new(name: &str, prefix: &str, meta_id: Option<ObjectId>) -> Self {
        Object {
            name: name.to_string(),
            entities: HashMap::new(),
            id: ObjectId::new(prefix),
            meta_id: meta_id,
            _marker: PhantomData,
        }
    }

    /// Updates an entity in the object or adds it if not present.e
    pub fn update_entity(&mut self, name: &str, attribute: A) {
        let entity = E::new(name, attribute);
        self.entities.insert(name.to_string(), entity);
    }

    // Get ID form object
    pub fn get_object_id(&self) -> &ObjectId {
        &self.id
    }

    // Get name form object
    pub fn get_name(&self)->&str{
        &self.name
    }

    pub fn get_meta_id(&self)->&Option<ObjectId>{
        &self.meta_id
    }

    /// Clones the current object and applies modifications to its entities.
    ///
    /// # Arguments
    ///
    /// * `modif` - A `HashMap` containing entity names as keys and updated attributes as values.
    ///
    /// # Returns
    ///
    /// A new `Object` with the modifications applied.
    pub fn clone_and_update(&self, modif: HashMap<String, A>) -> Self {
        let mut new_object = self.clone();

        modif.into_iter().for_each(|(k, v)| {
            new_object.update_entity(&k, v);
        });

        new_object
    }
}
