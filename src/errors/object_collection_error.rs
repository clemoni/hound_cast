
#[derive(Debug)]
pub enum ObjectCollectionError {
    MissingObject(String),
    NoMatchingObject(String)
}