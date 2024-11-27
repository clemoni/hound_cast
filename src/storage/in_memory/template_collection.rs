use std::collections::HashMap;

use crate::core::{model::UniqueId, template::template::Template};

use super::errors::CollectionError;
use crate::core::model::unique_id::Identifier;


pub struct TemplateCollection<Template>{
    data:HashMap<UniqueId, Template>
}

impl TemplateCollection<Template>{

    fn new()->TemplateCollection<Template>{
        TemplateCollection { data: HashMap::new() }
    }

   
    fn get(&self, id:&UniqueId)->Result<&Template,CollectionError> {
        self.data
        .get(&id.clone())
        .ok_or_else(|| CollectionError::MissingObject(id.get_id().to_string()))
    }

    
    fn insert(&mut self, value:&Template) {
        self.data.insert(value.get_id().clone(), value.clone());
    }

   
    fn remove(&mut self, id:&UniqueId)->Result<(),CollectionError> {
        self.data
        .remove(&id.get_id())
        .ok_or_else(|| CollectionError::MissingObject(id.to_string()))
        .and_then(|_| Ok(()))
    }

    fn get_by_parent_object(&self, parent_object_id:&UniqueId)->Result<Vec<&Template>, CollectionError>{
        let ouput = self
        .data
        .iter()
        .filter_map(|(_, template)|if template.get_parent_object()==parent_object_id {Some(template)} else {None} )
        .collect::<Vec<&Template>>();

        if ouput.is_empty(){
            Err(CollectionError::NoParentObject(parent_object_id.to_string()))
        }else{
            Ok(ouput)
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::core::{meta::meta_object::MetaObject, template::{self, template_builder::TemplateBuilder}};

    use super::*;

    #[test]
    fn test_insert_and_get_object() {
        let mut template_collection=TemplateCollection::new();

        let meta_obj = MetaObject::new_meta("TestObject").unwrap();

        let content=r#"This is a test content"#;

        let template=TemplateBuilder::instanciate(content, &meta_obj).build().unwrap();

        let template_id=template.get_id();

        template_collection.insert(&template);
        let retrieved_template = template_collection.get(&template_id);

        assert!(retrieved_template.is_ok());
        assert_eq!(retrieved_template.unwrap().get_id(), template_id);
 
    }
}