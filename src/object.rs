//! The `object` module provides the `Object` struct and related implementations.
//!
//! An `Object` represents a collection of entities, either in meta form (defining a schema)
//! or instance form (populated with real values).


use std::{collections::HashMap, marker::PhantomData};

use uuid::Uuid;

use crate::entity::EntityTraits;


#[derive(Debug, Clone)]
/// Represents a collection of entities.
/// 
/// This can be either a `MetaObject` defining the schema or an `InstanceObject` containing real data.
pub struct Object<E: EntityTraits<A>, A> {
    pub name: String,
    pub entities: HashMap<String, E>,
    pub id: String,
    pub meta_name:Option<String>,
    pub _marker: PhantomData<A>,
}

impl<E: EntityTraits<A>+Clone, A:Clone> Object<E, A> {

    /// Creates a new object with the given name and ID prefix.
    pub fn new(name: &str, prefix: &str, meta_name:Option<String>) -> Self {
        Object {
            name: name.to_string(),
            entities: HashMap::new(),
            id: format!("{}_{}", prefix, Uuid::new_v4()),
            meta_name,
            _marker: PhantomData,
        }
    }

    /// Updates an entity in the object or adds it if not present.e
    pub fn update_entity(&mut self, name: &str, attribute: A) {
        let entity = E::new(name, attribute);
        self.entities.insert(name.to_string(), entity);
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
    pub fn clone_and_update(&self, modif: HashMap<String, A>)->Self{
        let mut new_object=self.clone();

        modif
        .into_iter()
        .for_each(|(k,v)|{
            new_object.update_entity(&k, v);
        });

        new_object
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use std::collections::HashMap;

    
}