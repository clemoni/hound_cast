use hound_cast::{core::{instance::instance_object::InstanceObjectBuilder, meta::{meta_entity::MetaAttributes, meta_object::MetaObject}, model::UniqueId, template::template_builder::TemplateBuilder}, storage::in_memory::object_collections::{InstanceObjectCollection, MetaObjectCollection}};
use hound_cast::core::model::unique_id::Identifier;


fn main() {
    MetaObjectCollection::new();

    println!("{}",UniqueId::new("instance", None).unwrap().get_id());
    let mut meta_object_collection = MetaObjectCollection::new();

    let mut instance_object_collection = InstanceObjectCollection::new();
    
    let mut marathon = MetaObject::new_meta("Marathon").unwrap();
    marathon.update_entity("prize", MetaAttributes::I16);
    marathon.update_entity("ref_link", MetaAttributes::Text);
   
    meta_object_collection.insert(&marathon);

    let mut marathon_paris_build = InstanceObjectBuilder::new(&marathon, "Paris Marathon");
    marathon_paris_build.update_entity("prize", Some("2030")).unwrap_or_else(|err|println!("{}",err));
    marathon_paris_build.update_entity("ref_link", Some("link to ref")).unwrap_or_else(|err|println!("{}",err));
    marathon_paris_build.populate_missing_meta_entites();
    let marathon_paris = marathon_paris_build.build().unwrap();
    
    instance_object_collection.insert(&marathon_paris);

    let content = r#"This is a test with [@prize] and [@ref_link]"#;
    let template = TemplateBuilder::instanciate(&content, &marathon).build().unwrap();

    let populated_content = template.build_from_instance(&marathon_paris);
    println!("{:?}", populated_content);
}