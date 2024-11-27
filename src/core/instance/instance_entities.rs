use core::fmt;

use crate::core::{errors::AttributeError, model::entity::Entity};


pub type InstanceEntity = Entity<InstanceAttributes>;

#[derive(Debug, Clone, PartialEq)]
/// Enum representing the actual values for various attribute types in an instance object.
///
/// These are initialized based on the meta attributes and populated with real data.
pub enum InstanceAttributes {
    Text(Option<String>),
    I16(Option<i16>),
}

impl fmt::Display for InstanceAttributes{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self{
            InstanceAttributes::Text(value) =>{
                match value{
                    Some(t) => write!(f,"{}",t),
                    None => write!(f,"Null"),
                }
            },
            InstanceAttributes::I16(value) => match value{
                Some(i) => write!(f,"{}",i),
                None => write!(f,"Null"),
            },
        }
    }
}

impl InstanceAttributes {
    /// Parses an optional string input into a `Text` attribute.
    pub fn parse_text(input: Option<&str>) -> Result<InstanceAttributes, AttributeError> {
        match input {
            Some(txt) => Ok(InstanceAttributes::Text(Some(txt.to_string()))),
            None => Ok(InstanceAttributes::Text(None)),
        }
    }

    /// Parses an optional string input into an `I16` attribute.
    pub fn parse_i16(input: Option<&str>) -> Result<InstanceAttributes, AttributeError> {
        match input {
            Some(i16) => i16
                .parse::<i16>()
                .map(|num| InstanceAttributes::I16(Some(num)))
                .map_err(|_| AttributeError::InvalidType(format!("Expect i16 got {}", i16))),
            None => Ok(InstanceAttributes::I16(None)),
        }
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_success_parse_text() {
        let output = InstanceAttributes::parse_text(Some("test"));
        assert_eq!(
            output,
            Ok(InstanceAttributes::Text(Some("test".to_string())))
        );
    }

    #[test]
    fn test_success_parse_text_none() {
        let output = InstanceAttributes::parse_text(None);
        assert_eq!(output, Ok(InstanceAttributes::Text(None)));
    }

    #[test]
    fn test_success_parse_i16() {
        let output = InstanceAttributes::parse_i16(Some("33"));
        assert_eq!(output, Ok(InstanceAttributes::I16(Some(33))));
    }

    #[test]
    fn test_success_parse_i16_none() {
        let output = InstanceAttributes::parse_i16(None);
        assert_eq!(output, Ok(InstanceAttributes::I16(None)));
    }

    #[test]
    fn test_success_parse_i16_failed() {
        let output = InstanceAttributes::parse_i16(Some("failed"));
        assert_eq!(
            output,
            Err(AttributeError::InvalidType(
                "Expect i16 got failed".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_i16_none() {
        let result = InstanceAttributes::parse_i16(None);
        assert_eq!(result, Ok(InstanceAttributes::I16(None)));
    }

}
