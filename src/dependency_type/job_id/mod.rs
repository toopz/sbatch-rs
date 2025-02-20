//! JobId type and related functions
//!
//! The `JobId` enum represents a job id in sbatch. It can be either a number or a variable.
use std::fmt::Display;
use std::num::NonZeroU32;
use std::str::FromStr;

use thiserror::Error;

mod variable;

pub use variable::{Variable, VariableError};

/// Represents a job id in sbatch
///
/// The `JobId` enum represents a job id in sbatch. It can be either a number or a variable.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobId {
    /// A job id represented as a number, e.g. 1234
    Number(NonZeroU32),
    /// A job id represented as a variable, e.g. $step1
    Variable(Variable),
}

/// Error type for the `JobId` enum
///
/// The `JobIdError` enum represents all possible errors that can occur when working with the `JobId` enum.
#[derive(Debug, Error, PartialEq)]
pub enum JobIdError {
    #[error("Failed to parse JobId from string: {0}")]
    ParseError(String),
}

impl Display for JobId {
    /// Formats the `JobId` enum as a string.
    ///
    /// # Returns
    ///
    /// A string that represents the job id.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobId::Number(number) => write!(f, "{}", number),
            JobId::Variable(variable) => write!(f, "{}", variable),
        }
    }
}

impl From<NonZeroU32> for JobId {
    /// Converts a `NonZeroU32` into a `JobId`
    ///
    /// # Arguments
    ///
    /// * `value` - A `NonZeroU32` that holds the job id
    ///
    /// # Returns
    ///
    /// A `JobId` enum with the job id stored as a `JobId::Number`
    fn from(value: NonZeroU32) -> Self {
        JobId::Number(value)
    }
}

impl From<Variable> for JobId {
    /// Converts a `Variable` into a `JobId`
    ///
    /// # Arguments
    ///
    /// * `value` - A `Variable` that holds the job id
    ///
    /// # Returns
    ///
    /// A `JobId` enum with the job id stored as a `JobId::Variable`
    fn from(value: Variable) -> Self {
        JobId::Variable(value)
    }
}

impl FromStr for JobId {
    type Err = JobIdError;

    /// Parses a string into a `JobId`
    ///
    /// The input string can be either a number or a variable. If the input
    /// string is a number, it will be parsed into a `NonZeroU32` and stored
    /// as a `JobId::Number`. If the input string is a variable, it will be
    /// parsed into a `Variable` and stored as a `JobId::Variable`.
    ///
    /// Note: the JobId::Number and JobId::Variable variants are mutually
    /// exclusive, so it is impossible to have an input string that can be
    /// parsed into both variants.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the input string
    ///
    /// # Returns
    ///
    /// A Result containing a `JobId` if the input string can be parsed, or a
    /// `JobIdError` if the input string cannot be parsed.
    ///
    /// # Errors
    ///
    /// Returns a `JobIdError` if the input string cannot be parsed into a `JobId`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Trim the input string to remove quotes and spaces
        let s = s.trim_matches('\'').trim_matches('"').trim();

        // Try to parse the input string as a number, if it fails, try to parse it as a variable
        NonZeroU32::from_str(s)
            .map(JobId::Number)
            .or_else(|_| Variable::from_str(s).map(JobId::Variable))
            .map_err(|_| JobIdError::ParseError(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_from_number() {
        let number = NonZeroU32::try_from(1).unwrap();
        let job_id_type: JobId = number.clone().into();
        assert_eq!(job_id_type, JobId::Number(number));
    }

    #[test]
    fn test_from_variable() {
        let variable = Variable::new("variable").unwrap();
        let job_id_type: JobId = variable.clone().into();
        assert_eq!(job_id_type, JobId::Variable(variable));
    }

    #[rstest]
    #[case("1", JobId::Number(NonZeroU32::try_from(1).unwrap()))]
    #[case(r#""1""#, JobId::Number(NonZeroU32::try_from(1).unwrap()))]
    #[case(r#"'1'"#, JobId::Number(NonZeroU32::try_from(1).unwrap()))]
    #[case("$variable", JobId::Variable(Variable::new("variable").unwrap()))]
    #[case("${variable}", JobId::Variable(Variable::new("variable").unwrap()))]
    #[case(r#""$variable""#, JobId::Variable(Variable::new("variable").unwrap()))]
    fn test_from_str(#[case] input: &str, #[case] expected: JobId) {
        let result: JobId = input.parse().unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("invalid")]
    #[case("")]
    fn test_from_str_error(#[case] input: &str) {
        let result: Result<JobId, JobIdError> = input.parse();
        assert!(result.is_err());
    }
}
