//! This module contains the implementation of the `Variable` struct which represents a bash variable.
//!
//! The `Variable` struct is a wrapper around the `Name` struct and provides a way to parse a string
//! into a valid bash variable. The `Variable` struct implements the `FromStr` trait to allow parsing
//! a string into a `Variable` struct.
//!
//! # Examples
//!
//! ```
//! use std::str::FromStr;
//! use sbatch_rs::Variable;
//!
//! // Create a new variable
//! let variable = Variable::new("name").unwrap();
//! assert_eq!(variable.to_string(), "${name}");
//!
//! // The variable name must be a valid bash variable
//! let variable = Variable::new("1name");
//! assert!(variable.is_err());
//!
//! // from_str() assumes format of ${name} or $name
//! let variable = Variable::from_str("${name}").unwrap();
//! assert_eq!(variable.to_string(), "${name}");
//!
//! let variable = Variable::from_str("$name").unwrap();
//! assert_eq!(variable.to_string(), "${name}");
//!
//! let variable = Variable::from_str("name");
//! assert!(variable.is_err());
//!
//! ```

use std::fmt::Display;
use std::str::FromStr;

use regex::Regex;
use thiserror::Error;

mod name;
use name::Name;

/// Represents a bash variable
///
/// The `Variable` struct represents a bash variable and is a wrapper around the `Name` struct.
///
/// # Examples
///
/// ```
/// use sbatch_rs::Variable;
///
/// // Create a new variable
/// let variable = Variable::new("name").unwrap();
/// assert_eq!(variable.to_string(), "${name}");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variable(Name);

/// Error type for the `Variable` struct
///
/// The `VariableError` enum represents all possible errors that can occur when working with the `Variable` struct.
///
/// # Examples
///
/// ```
/// use sbatch_rs::VariableError;
///
/// let error = VariableError::InvalidName("1name".to_string());
/// assert_eq!(error.to_string(), "Invalid variable name: '1name'");
///
/// let error = VariableError::ParseError("${name}".to_string());
/// assert_eq!(error.to_string(), "Failed to parse variable from string: ${name}");
/// ```
#[derive(Debug, Error)]
pub enum VariableError {
    #[error("Invalid variable name: '{0}'")]
    InvalidName(String),
    #[error("Failed to parse variable from string: {0}")]
    ParseError(String),
}

impl Variable {
    /// Creates a new `Variable` after validating the input string.
    ///
    /// # Arguments
    /// 
    /// * `value` - An object that can be converted into a string
    /// 
    /// # Errors
    ///
    /// Returns a `VariableError` if the input string is empty, contains invalid characters,
    /// starts with a number, or is a reserved name.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Variable;
    ///
    /// let variable = Variable::new("name").unwrap();
    /// assert_eq!(variable.to_string(), "${name}");
    ///
    /// let variable = Variable::new("1name");
    /// assert!(variable.is_err());
    /// ```
    pub fn new(value: impl ToString) -> Result<Self, VariableError> {
        // Check if the variable name is valid, if not return an error
        Name::new(value)
            .map(Variable)
            .map_err(|e| VariableError::InvalidName(e.to_string()))
    }

    /// Parses the input string using the provided regex pattern
    ///
    /// # Errors
    ///
    /// Returns a `VariableError` if the input string cannot be parsed using the provided regex pattern.
    fn parse_with_regex(value: &str, pattern: &str) -> Result<Option<String>, VariableError> {
        // Try to parse the input string using the provided regex pattern
        let re = Regex::new(pattern).map_err(|e| VariableError::ParseError(e.to_string()))?;

        // If the regex pattern matches the input string, return the variable name
        Ok(re.captures(value).and_then(|captures| {
            captures
                .get(1)
                .map(|var_name| var_name.as_str().to_string())
        }))
    }
}

impl FromStr for Variable {
    type Err = VariableError;

    /// Parses a string into a `Variable`.
    ///
    /// This method is used to create a `Variable` from a string by validating the input.
    ///
    /// # Errors
    ///
    /// Returns a `VariableError` if the input string is empty, contains invalid characters,
    /// starts with a number, or is a reserved name.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sbatch_rs::Variable;
    ///
    /// let variable = Variable::from_str("${name}").unwrap();
    /// assert_eq!(variable.to_string(), "${name}");
    ///
    /// let variable = Variable::from_str("$name").unwrap();
    /// assert_eq!(variable.to_string(), "${name}");
    ///
    /// let variable = Variable::from_str("name");
    /// assert!(variable.is_err());
    /// ```
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // Trim the input string to remove quotes and spaces
        let trimmed_value = value.trim_matches('\'').trim_matches('"').trim();

        // Try to parse the input string assuming: ${name} or $name
        match Self::parse_with_regex(trimmed_value, r"^\$\{(.+)\}$")? {
            Some(var_name) => Variable::new(var_name),
            None => match Self::parse_with_regex(trimmed_value, r"^\$(.+)$")? {
                Some(var_name) => Variable::new(var_name),
                None => Err(VariableError::ParseError(value.to_string())),
            },
        }
    }
}

impl Display for Variable {
    /// Formats the `Variable` as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Variable;
    ///
    /// let variable = Variable::new("name").unwrap();
    /// assert_eq!(variable.to_string(), "${name}");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${{{}}}", self.0)
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
        let name = Variable::new(value);
        assert!(name.is_err());
    }

    #[rstest]
    #[case("name")]
    #[case("name1")]
    #[case("name_")]
    #[case("name_1")]
    fn test_new_is_ok(#[case] value: &str) {
        let bash_variable = Variable::new(value);
        if bash_variable.is_err() {
            panic!("Failed to create Variable: {:?}", bash_variable.err());
        }
        assert!(bash_variable.is_ok());
    }

    #[rstest]
    #[case("${name}", "name")]
    #[case("${name1}", "name1")]
    #[case("${name_}", "name_")]
    #[case("${name_1}", "name_1")]
    #[case("$name", "name")]
    #[case("$name1", "name1")]
    #[case("$name_", "name_")]
    #[case("$name_1", "name_1")]
    #[case(r#""$name_1""#, "name_1")]
    fn test_parse(#[case] value: &str, #[case] expected: &str) {
        let variable = Variable::from_str(value).unwrap();
        assert_eq!(variable.0.to_string(), expected);
    }

    #[rstest]
    #[case("${name$}")]
    #[case("${name1}$")]
    #[case("$${name_}")]
    #[case("${$name_1}")]
    #[case("$name$")]
    #[case("$$name1")]
    #[case("$name/blah")]
    #[case("name")]
    #[case("name1")]
    #[case("name$nope")]
    fn test_parse_error(#[case] value: &str) {
        let variable = Variable::from_str(value);
        assert!(variable.is_err());
    }

    #[test]
    fn test_variable_display() {
        let variable = Variable::new("name").unwrap();
        assert_eq!(variable.to_string(), "${name}");
    }
}
