use std::collections::HashMap;

use errors::attribute_error::AttributeError;
use instance::{instance_collection::InstanceObjectCollection, instance_object::InstanceObjectBuilder};
use meta::{meta_collection::MetaObjectCollection, meta_entity::MetaAttributes, meta_object::MetaObject};
use template::template_builder::TemplateBuilder;


pub mod model;
pub mod instance;
pub mod meta;
pub mod errors;
pub mod template;



// template 
// link to an object
// create a text with @variables 
// if empty will pop input for user
// 

// struct Template{
//     content:String 
// }

// impl Template{
//     fn cleaned_word(word:&str)->&str{
//         word.trim_matches(|c: char| !c.is_alphanumeric() && c != '@' && c != '[' && c != ']')
//     }

//     fn match_entities<A,E>(word:&str, object_entities:HashMap<String, E>)->String
//     where E:EntityTraits<A>
//     {

//         let re = Regex::new(r"\[@(?P<reference>[^\]]+)\]").unwrap();
//         let cleaned_word=Template::cleaned_word(word);
//         match re.captures(cleaned_word) {
//             Some(capture_word) =>match object_entities.get(&capture_word["reference"]){
//                 Some(match_word) =>word.replace(&capture_word[0], match_word.).to_string(),
//                 None => word.to_string(),
//             },
//             None => word.to_string(),
//         }

//     }
// }

// template
// belong to object


// template_instance_holder
// templace
// hashmap? <String, Collection>

//  


fn main()->Result<(),AttributeError>{
    let mut meta_object_collection=MetaObjectCollection::new();
    let mut instance_object_collection=InstanceObjectCollection::new();
    
    let mut marathon = MetaObject::new_meta("Marathon");
    marathon.update_entity("prize", MetaAttributes::I16);
    marathon.update_entity("ref_link", MetaAttributes::Text);
   

    meta_object_collection.insert(&marathon);


    let mut marathon_paris_build=InstanceObjectBuilder::new(&marathon, "Paris Marathon");
    marathon_paris_build.update_entity("prize", Some("2030")).unwrap_or_else(|err|println!("{}",err));
    marathon_paris_build.update_entity("ref_link",Some("link to ref")).unwrap_or_else(|err|println!("{}",err));
    marathon_paris_build.populate_missing_meta_entites();
    let marathon_paris=marathon_paris_build.build();
    
    instance_object_collection.insert(&marathon_paris);

    let mut marathon_belfast_build=InstanceObjectBuilder::new(&marathon, "Belfast Marathon");
    marathon_belfast_build.update_entity("prize", Some("304")).unwrap_or_else(|err|println!("{}",err));
    marathon_belfast_build.update_entity("ref_link",Some("http//link_to_belfast")).unwrap_or_else(|err|println!("{}",err));
    marathon_belfast_build.populate_missing_meta_entites();
    let marathon_belfast=marathon_belfast_build.build();
    
    instance_object_collection.insert(&marathon_belfast);


    let content=r#"This is a test with [@prize] and [@ref_link]"#;

    let template=TemplateBuilder::instanciate(&content, &marathon).build().unwrap();

    let populated_content=template.build_from_instance(&marathon_belfast);


   println!("{:?}",populated_content);

    


    // println!("{:?}", out);

    // instance_object_collection.list_objects();


    // let re = Regex::new(r"\[@(?P<reference>[^\]]+)\]").unwrap();
   
    
    
    // let base_text=r#"Lorem Ipsum [@prize]€ simply dummy text of the printing and typesetting [@ref_link]."#;


    // let t = base_text
    //     .split_whitespace()
    //     .map(|word| {
    //         let cleaned_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '@' && c != '[' && c != ']');
    //         if let Some(capture) = re.captures(cleaned_word) {

    //             match coll.get(&capture["reference"]) {
    //                 Some(value) => {
    //                     println!("{} {}",&capture[0], value);
    //                     word.replace(&capture[0], value)
    //                 },
    //                 None => word.to_string(),
    //             }
    //         } else {
    //             word.to_string()
    //         }
    //     })
    //     .collect::<Vec<String>>()
    //     .join(" ");

    // println!("{:?}",t);

    Ok(())


}
