//! # Template Module
//!
//! The `Template` module provides functionality to create and manage templates for dynamic content
//! generation. Templates are populated using attributes from associated `InstanceObject`.
//!
//! ## Core Components:
//!
//! ### `TemplateContent`
//! Encapsulates the raw template string and provides methods for content validation and transformation.
//!
//! ### `Template`
//! Represents a specific template tied to a parent `ObjectId`. Responsible for building the final content by
//! injecting values from `InstanceObject` attributes into the placeholders.
//!
//! ### `TemplateBuilder`
//! Facilitates the creation of a `Template`, validating that all required entities are present in the template content.
//!
//! ## Template Syntax:
//! - Placeholders in templates follow the format: `[@attribute_name]`.
//! - The `attribute_name` must match the entity names in the associated `MetaObject`.
//!
//! ## Error Handling:
//! The module defines `TemplateError` for various error scenarios:
//! - `MissingMetaObjectId`: Raised when an `InstanceObject` lacks a parent `MetaObject` ID.
//! - `UnauthrorisedActionFromMetaObject`: Raised when a `Template` is used with an incompatible `InstanceObject`.
//! - `MissingEntitiesFromMetaObject`: Raised when required entities are missing from the `InstanceObject`.

pub mod template;
pub mod template_content;
pub mod template_builder;