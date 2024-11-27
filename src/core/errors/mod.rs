//! # Errors Module
//!
//! This module centralizes error handling for the application. Each error type is defined in a separate
//! submodule, following specific responsibilities, and re-exported here for convenient access.


pub mod attribute_error;
pub mod object_collection_error;
pub mod object_error;
pub mod template_error;
pub mod unique_id_errors;

pub use attribute_error::AttributeError;
pub use object_collection_error::ObjectCollectionError;
pub use object_error::ObjectError;
pub use template_error::TemplateError;
pub use unique_id_errors::UniqueIdError;