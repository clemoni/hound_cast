use crate::model::object::Object;

use super::meta_entity::{MetaAttributes, MetaEntity};



pub type MetaObject = Object<MetaEntity, MetaAttributes>;

impl MetaObject {
    pub fn new_meta(name: &str) -> Self {
        Object::new(name, "meta", None)
    }
   
}



#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_new_meta_object_creation() {
        let meta_object = MetaObject::new_meta("TestMetaObject");

        assert_eq!(meta_object.name, "TestMetaObject");
        assert!(meta_object.entities.is_empty());
        assert!(meta_object.id.get_id().starts_with("meta_"));
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