//! # TemplateContent
//!
//! The `TemplateContent` struct wraps around the raw template string. It provides:
//! - Placeholder pattern matching using `Regex`.
//! - Methods to validate and extract entities from the template.


use std::collections::HashSet;
use regex::Regex;

use crate::{core::model::{entity::EntityTraits, object::Object}, core::errors::TemplateError};


/// Encapsulates the raw content of a template.
#[derive(Debug,Clone)]
pub struct TemplateContent(String);

impl TemplateContent {
    /// Creates a new `TemplateContent`.
    pub fn new(content: &str) -> Self {
        TemplateContent(content.to_string())
    }

    /// Returns the content as a string slice.
    pub fn get_content(&self) -> &str {
        &self.0
    }

    /// Returns the regular expression pattern for template placeholders.
    pub fn get_pattern() -> Regex {
        Regex::new(r"\[@(?P<reference>[^\]]+)\]").unwrap()
    }

    /// Cleans a word by stripping any non-alphanumeric characters, except for template-specific symbols.
    pub fn clean_word(word: &str) -> &str {
        word.trim_matches(|c: char| !c.is_alphanumeric() && c != '@' && c != '[' && c != ']')
    }

    /// Validates that all entities in the `Object` are referenced in the template.
    pub fn is_matching_entity<E: EntityTraits<A>, A>(
        &self,
        object: &Object<E, A>,
    ) -> Result<(), TemplateError> {
        let content_entities: HashSet<_> = Self::get_pattern()
            .captures_iter(&self.0)
            .filter_map(|caps| caps.name("reference").map(|m| m.as_str()))
            .collect();

        let object_entities: HashSet<&str> = object.entities.keys().map(|k| k.as_str()).collect();

        if content_entities == object_entities {
            Ok(())
        } else {
            Err(TemplateError::MissingEntitiesFromMetaObject(
                object_entities
                    .difference(&content_entities)
                    .map(|v| v.to_string())
                    .collect(),
            ))
        }
    }
}