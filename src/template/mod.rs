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
//!
//! ## Example Usage:
//! ```rust
//! use template::template_builder::TemplateBuilder;
//! use template::template::TemplateError;
//! use meta::meta_object::MetaObject;
//! use instance::instance_object::InstanceObjectBuilder;
//!
//! let mut meta_object = MetaObject::new_meta("ExampleMeta");
//! meta_object.update_entity("title", MetaAttributes::Text);
//! meta_object.update_entity("year", MetaAttributes::I16);
//!
//! let mut instance_builder = InstanceObjectBuilder::new(&meta_object, "ExampleInstance");
//! instance_builder.update_entity("title", Some("Rust Guide")).unwrap();
//! instance_builder.update_entity("year", Some("2023")).unwrap();
//! let instance_object = instance_builder.build();
//!
//! let template_content = "Title: [@title], Year: [@year]";
//! let template = TemplateBuilder::instanciate(&template_content, &meta_object)
//!     .build()
//!     .expect("Failed to build template");
//!
//! let result = template.build_from_instance(&instance_object).unwrap();
//! assert_eq!(result, "Title: Rust Guide, Year: 2023");
//! ```
//!
//! The module is divided into the following submodules:
//! - `template.rs`: Core `Template` struct and related logic.
//! - `template_content.rs`: Encapsulates template string and validation methods.
//! - `template_builder.rs`: Builder pattern for creating `Template` instances.

pub mod template;
pub mod template_content;
pub mod template_builder;