use crate::model::object_collection::ObjectCollection;

use super::meta_entity::{MetaAttributes, MetaEntity};

pub type MetaObjectCollection=ObjectCollection<MetaEntity, MetaAttributes>;
