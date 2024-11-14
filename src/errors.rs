//! This module defines errors that may occur during attribute handling.
//!
//! The `AttributeError` enum provides descriptive error variants for issues like type mismatches or invalid input.

use std::fmt;

#[derive(Debug, PartialEq)]
/// Enum representing errors that can occur during attribute parsing and validation.
pub enum AttributeError{
    InvalidType(String),
    NonMatchingType(String)
}

impl fmt::Display for AttributeError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self{
            AttributeError::InvalidType(err) => write!(f, "Error {}",err),
            AttributeError::NonMatchingType(err) => write!(f, "Error {}",err),
        }
    }
}