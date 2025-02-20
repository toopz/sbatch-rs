//! This module provides a `Name` type that represents a validated name.
//!
//! A `Name` must be non-empty, can only contain alphanumeric characters or
//! underscores, cannot start with a number, and cannot be a reserved name.
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

mod reserved_names;
use reserved_names::RESERVED_NAMES;

/// Errors that can occur when validating a name.
#[derive(Debug, Error)]
pub enum NameError {
    /// The name cannot be empty.
    #[error("Name cannot be empty, omit field if not needed")]
    EmptyName,
    /// The name contains invalid characters.
    #[error("Name can only contain alphanumeric characters or underscores, got '{0}'")]
    InvalidCharacters(String),
    /// The name starts with a number.
    #[error("Name cannot start with a number, got '{0}'")]
    StartsWithNumber(String),
    /// The name is reserved and cannot be used.
    #[error("Name '{0}' is reserved and cannot be used")]
    ReservedName(String),
}

/// Represents a validated name.
///
/// A `Name` must be non-empty, can only contain alphanumeric characters or
/// underscores, cannot start with a number, and cannot be a reserved name.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(String);

impl Name {
    /// Creates a new `Name` after validating the input string.
    ///
    /// # Arguments
    ///
    /// * `value` - An object that can be converted into a string
    ///
    /// # Errors
    ///
    /// Returns a `NameError` if the input string is empty, contains invalid characters,
    /// starts with a number, or is a reserved name.
    pub fn new(value: impl ToString) -> Result<Self, NameError> {
        // Convert the input value to a string
        let value = value.to_string();

        // Validate the name
        if value.is_empty() {
            return Err(NameError::EmptyName);
        }
        if !value.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(NameError::InvalidCharacters(value));
        }
        if value.chars().next().unwrap().is_numeric() {
            return Err(NameError::StartsWithNumber(value));
        }
        if RESERVED_NAMES.contains(&value.as_str()) {
            return Err(NameError::ReservedName(value));
        }

        // Success, return the validated name
        Ok(Name(value))
    }
}

impl Display for Name {
    /// Formats the `Name` for display.
    ///
    /// This method is used to convert the `Name` into a string for display purposes.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Name {
    type Err = NameError;

    /// Parses a string into a `Name`.
    ///
    /// This method is used to create a `Name` from a string by validating the input.
    ///
    /// # Errors
    ///
    /// Returns a `NameError` if the input string is empty, contains invalid characters,
    /// starts with a number, or is a reserved name.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Name::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("")]
    #[case("1")]
    #[case("alias")]
    #[case("1name")]
    #[case("name!")]
    #[case("name@")]
    #[case("name#")]
    #[case("name$")]
    #[case("name%")]
    #[case("name^")]
    #[case("name&")]
    #[case("name*")]
    #[case("name(")]
    #[case("name)")]
    #[case("name-")]
    #[case("name=")]
    #[case("name+")]
    #[case("name[")]
    #[case("name]")]
    #[case("name{")]
    #[case("name}")]
    #[case("name|")]
    #[case("name\\")]
    #[case("name;")]
    #[case("$name")]
    #[case("name?")]
    #[case("name'")]
    #[case("name\"")]
    #[case("name:")]
    #[case("name<")]
    #[case("name>")]
    #[case("name,")]
    fn test_new_is_error(#[case] value: &str) {
        let name = Name::new(value);
        assert!(name.is_err());
    }

    #[rstest]
    #[case("name")]
    #[case("name1")]
    #[case("name_")]
    #[case("name_1")]
    fn test_new_is_ok(#[case] value: &str) {
        // Using new()
        let bash_variable = Name::new(value);
        if bash_variable.is_err() {
            panic!("Failed to create Name: {:?}", bash_variable.err());
        }
        assert!(bash_variable.is_ok());

        let name = bash_variable.unwrap();
        assert_eq!(name.to_string(), value);

        // Using from_str()
        let bash_variable = Name::from_str(value);
        if bash_variable.is_err() {
            panic!("Failed to create Name: {:?}", bash_variable.err());
        }
        assert!(bash_variable.is_ok());
    }

    #[test]
    fn test_name_reserved_keywords() {
        for keyword in RESERVED_NAMES.iter() {
            let name = Name::new(keyword);
            assert!(name.is_err());
        }
    }

    #[rstest]
    #[case("name")]
    #[case("name1")]
    #[case("name_")]
    fn test_display(#[case] value: &str) {
        let name = Name::new(value).unwrap();
        let result = format!("{}", name);
        assert_eq!(result, value);
    }
}
