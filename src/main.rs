use errors::AttributeError;
use instance::InstanceObjectBuilder;
use meta::{MetaAttributes, MetaObject};


pub mod errors;
pub mod meta;
pub mod instance;
pub mod entity;
pub mod object;


fn main()->Result<(),AttributeError>{
    let mut marathon = MetaObject::new_meta("Marathon");
    marathon.update_entity("prize", MetaAttributes::I16);
    marathon.update_entity("ref_link", MetaAttributes::Text);
    marathon.update_entity("test", MetaAttributes::Text);

    println!("{:#?}", marathon);

    let mut marathon_paris_build=InstanceObjectBuilder::new(marathon, "Paris Marathon");
    marathon_paris_build.update_entity("prize", Some("2030")).unwrap_or_else(|err|println!("{}",err));
    marathon_paris_build.update_entity("ref_link",Some("link to ref")).unwrap_or_else(|err|println!("{}",err));
    marathon_paris_build.populate_missing_meta_entites();
    let marathon_paris=marathon_paris_build.build();
    
    println!("{:#?}",marathon_paris);

    Ok(())


}
