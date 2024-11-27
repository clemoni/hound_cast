//! This module defines errors that may occur during attribute handling.
//!
//! The `AttributeError` enum provides descriptive error variants for issues like type mismatches or invalid input.

use std::fmt;

use thiserror::Error;

#[derive(Debug, PartialEq,Error)]
/// Enum representing errors that can occur during attribute parsing and validation.
pub enum AttributeError{
    #[error("AttributeError: Invalid Type {0}")]
    InvalidType(String),
    #[error("AttributeError: None matching type {0}")]
    NonMatchingType(String)
}

