//! This module defines the `Entity` struct and `EntityTraits` trait.
//!
//! The `Entity` struct represents a generic entity with a name and an attribute.
//! The `EntityTraits` trait ensures a consistent interface for creating and interacting
//! with entities across both meta and instance objects.

pub trait EntityTraits<A> {
    /// Creates a new entity with the specified name and attribute.
    fn new(name: &str, attribute: A) -> Self; 

    /// Retrieves the name of the entity.
    fn get_name(&self) -> &str;

    /// Retrieves the attribute associated with the entity.
    fn get_attribute(&self) -> &A;
}

#[derive(Debug, Clone)]
/// A generic struct representing an entity.
/// 
/// Each entity has a name and an associated attribute of type `A`.
pub struct Entity<A> {
    name: String,
    attribute: A,
}

impl<A> EntityTraits<A> for Entity<A> {
    fn new(name: &str, attribute: A) -> Self {
        Entity {
            name: name.to_string(),
            attribute,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_attribute(&self) -> &A {
        &self.attribute
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct MockAttribute(i32);

    type MockEntity=Entity<MockAttribute>;


    #[test]
    fn test_new_mock_creation(){
        let mock_entity=MockEntity::new("TestEntity", MockAttribute(23));;
        assert_eq!(mock_entity.get_name(), "TestEntity");
        assert_eq!(mock_entity.get_attribute(), &MockAttribute(23));
    }
    
}
