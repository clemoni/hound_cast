use thiserror::Error;


#[derive(Debug,Error)]
pub enum ObjectError{
    #[error("ObjectError: Meta Object is Missing")]
    MissingMetaObject
}