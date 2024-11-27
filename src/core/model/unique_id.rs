//! UniqueId Module
//!
//! This module provides functionality for generating and parsing unique identifiers.
//! A `UniqueId` is composed of three parts: a prefix, a timestamp, and a random key.
//!

use std::fmt;
use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};

use crate::core::errors::UniqueIdError;

pub trait Identifier{
    fn get_id(&self)->&UniqueId;
}


#[derive(Debug,Clone)]
/// Enum representing the parts of a `UniqueId`.
pub enum UniqueIdParts {
    Prefix,
    Timestamp,
    Key,
}

#[derive(Debug,Clone,PartialEq,Eq, Hash)]
/// A struct representing a unique identifier composed of a prefix, a timestamp, and a random key.
pub struct UniqueId(String);

impl UniqueId {
    /// Returns the current timestamp in milliseconds since the Unix epoch.
    fn get_timestamp() -> i64 {
        Utc::now().timestamp_millis()
    }

    /// Generates a random alphanumeric key of the specified length.
    fn get_rand_key(length: Option<usize>) -> String {
        let length = length.unwrap_or(8);
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect::<String>()
    }

    /// Creates a new `UniqueId` with the given prefix and random key length.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix for the `UniqueId`. Must not be empty.
    /// * `length` - The length of the random key. Defaults to 8 if `None`.
    ///
    /// # Errors
    ///
    /// Returns a `UniqueIdError::MissingPrefix` if the prefix is empty.
    pub fn new(prefix: &str, length: Option<usize>) -> Result<UniqueId, UniqueIdError> {
        match prefix.is_empty() {
            false => {
                let ts = Self::get_timestamp();
                let rand_key = Self::get_rand_key(length);
                Ok(UniqueId(format!("{prefix}:{ts}:{rand_key}")))
            }
            true => Err(UniqueIdError::MissingPrefix),
        }
    }

    /// Parses a specific part of the `UniqueId`.
    ///
    /// # Arguments
    ///
    /// * `part` - The part of the `UniqueId` to parse (`Prefix`, `Timestamp`, or `Key`).
    ///
    /// # Errors
    ///
    /// Returns `UniqueIdError::WrongFormat` if the `UniqueId` format is invalid.
    pub fn parse(&self, part: UniqueIdParts) -> Result<String, UniqueIdError> {
        let parts = self.0.split(":").collect::<Vec<&str>>();
        match parts.len() == 3 {
            true => match part {
                UniqueIdParts::Prefix => Ok(parts[0].to_string()),
                UniqueIdParts::Timestamp => Ok(parts[1].to_string()),
                UniqueIdParts::Key => Ok(parts[2].to_string()),
            },
            false => Err(UniqueIdError::WrongFormat(parts.len())),
        }
    }
}

impl fmt::Display for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Identifier for UniqueId{
     /// Returns the string representation of the `UniqueId`.
    fn get_id(&self) -> &UniqueId {
        &self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_id_creation() {
        let unique_id = UniqueId::new("test", Some(10)).unwrap();
        assert!(unique_id.get_id().to_string().starts_with("test:"));
    }

    #[test]
    fn test_empty_prefix_error() {
        let result = UniqueId::new("", Some(10));
        assert!(matches!(result, Err(UniqueIdError::MissingPrefix)));
    }

    #[test]
    fn test_parse_prefix() {
        let unique_id = UniqueId::new("test", Some(10)).unwrap();
        let prefix = unique_id.parse(UniqueIdParts::Prefix).unwrap();
        assert_eq!(prefix, "test");
    }

    #[test]
    fn test_parse_timestamp() {
        let unique_id = UniqueId::new("test", Some(10)).unwrap();
        let timestamp = unique_id.parse(UniqueIdParts::Timestamp).unwrap();
        assert!(timestamp.parse::<i64>().is_ok());
    }

    #[test]
    fn test_parse_key() {
        let unique_id = UniqueId::new("test", Some(10)).unwrap();
        let key = unique_id.parse(UniqueIdParts::Key).unwrap();
        assert_eq!(key.len(), 10);
    }

    #[test]
    fn test_invalid_format_error() {
        let unique_id = UniqueId("invalid_format".to_string());
        let result = unique_id.parse(UniqueIdParts::Prefix);
        assert!(matches!(result, Err(UniqueIdError::WrongFormat(_))));
    }
}
