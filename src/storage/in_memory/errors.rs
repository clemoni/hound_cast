
#[derive(Debug)]
pub enum CollectionError {
    MissingObject(String),
    NoMatchingObject(String),
    NoParentObject(String)
}