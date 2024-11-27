//! The `object` module provides the `Object` struct and related implementations.
//!
//! An `Object` represents a collection of entities, either in meta form (defining a schema)
//! or instance form (populated with real values).

use std::{collections::HashMap, marker::PhantomData};

use crate::core::errors::UniqueIdError;

use super::{entity::EntityTraits, unique_id::Identifier, UniqueId};


#[derive(Debug, Clone)]
/// Represents a collection of entities.
///
/// This can be either a `MetaObject` defining the schema or an `InstanceObject` containing real data.
pub struct Object<E: EntityTraits<A>, A> {
    pub name: String,
    pub entities: HashMap<String, E>,
    pub id: UniqueId,
    pub meta_id: Option<UniqueId>,
    pub _marker: PhantomData<A>,
}

impl<E: EntityTraits<A> + Clone, A: Clone> Object<E, A> {
    /// Creates a new object with the given name and ID prefix.
    pub fn new(name: &str, prefix: &str, meta_id: Option<UniqueId>) -> Result<Self,UniqueIdError> {
        let unique_id_res=UniqueId::new(prefix,None);
        match unique_id_res{
            Ok(unique_id) => Ok(Object {
                name: name.to_string(),
                entities: HashMap::new(),
                id: unique_id,
                meta_id: meta_id,
                _marker: PhantomData,
            }),
            Err(err) => Err(err),
        }
    }

    /// Updates an entity in the object or adds it if not present.e
    pub fn update_entity(&mut self, name: &str, attribute: A) {
        let entity = E::new(name, attribute);
        self.entities.insert(name.to_string(), entity);
    }

    // // Get ID form object
    // pub fn get_object_id(&self) -> &UniqueId {
    //     &self.id
    // }

    // Get name form object
    pub fn get_name(&self)->&str{
        &self.name
    }

    pub fn get_meta_id(&self)->&Option<UniqueId>{
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

impl<E: EntityTraits<A> + Clone, A: Clone> Identifier for Object<E, A> {
    fn get_id(&self) -> &UniqueId {
        &self.id
    }
}