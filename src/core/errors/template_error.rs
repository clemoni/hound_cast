//! # Errors
//!
//! This module defines the `TemplateError` enum, which represents errors that can occur during
//! template creation and population.

use thiserror::Error;

/// Represents errors that can occur during template creation or population.
#[derive(Debug, PartialEq, Error)]
pub enum TemplateError {
    /// Raised when the `InstanceObject` lacks a `MetaObject` ID.
    #[error("TemplateError: Instance Object {0} doesn't have a parent Meta Object ID registered")]
    MissingMetaObjectId(String),

    /// Raised when the `InstanceObject` is not associated with the expected `MetaObject`.
    #[error("TemplateError: Unauthorized action from Meta Object {meta}, Instance Object {instance} is not its parent")]
    UnauthrorisedActionFromMetaObject {
        meta: String,
        instance: String,
    },

    /// Raised when template references entities missing from the `InstanceObject`.
    #[error("TemplateError: Following entities {0:?} are missing from the content being built")]
    MissingEntitiesFromMetaObject(Vec<String>),

    #[error("TemplateError: Propagated error: {0}")]
    PropagatedError(String)
}