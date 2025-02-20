//! Time delay for job dependencies.
//!
//! The "after" sbatch dependency allows for an optional time delay to be set. This time delay
//! is represented by the `TimeDelay` struct. The `TimeDelay` struct is a wrapper around a
//! `NonZeroU32` and provides a way to parse a string into a valid time delay. The `TimeDelay`
//! struct implements the `FromStr` trait to allow parsing a string into a `TimeDelay` struct.
//! TimeDelay can also be created from a `u32` using the `TryFrom` trait.

use std::fmt::Display;
use std::num::{NonZeroU32, ParseIntError, TryFromIntError};
use std::str::FromStr;

use thiserror::Error;

/// Represents a time delay for job dependencies.
///
/// The `TimeDelay` struct represents a time delay for job dependencies. It is a wrapper around
/// a `NonZeroU32` and provides a way to parse a string into a valid time delay. The `TimeDelay`
/// struct implements the `FromStr` trait to allow parsing a string into a `TimeDelay` struct.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDelay(NonZeroU32);

/// Error type for the `TimeDelay` struct
///
/// The `TimeDelayError` enum represents all possible errors that can occur when working with the `TimeDelay` struct.
#[derive(Debug, Error, PartialEq)]
pub enum TimeDelayError {
    /// Error when parsing a string into a `TimeDelay`
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] ParseIntError),
    /// Error when converting a `u32` into a `TimeDelay`
    #[error("TryFromIntError: {0}")]
    TryFromIntError(#[from] TryFromIntError),
}

impl Display for TimeDelay {
    /// Formats the `TimeDelay` struct as a string.
    ///
    /// # Returns
    ///
    /// A string that represents the time delay.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TimeDelay {
    type Err = TimeDelayError;

    /// Parses a string into a `TimeDelay`
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the time delay
    ///
    /// # Returns
    ///
    /// A `TimeDelay` struct with the time delay stored as a `NonZeroU32`.
    ///
    /// # Errors
    ///
    /// Returns a `TimeDelayError` if the input string is not a valid positive integer.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip quotes and spaces if present
        let s = s.trim_matches('"').trim_matches('\'').trim();

        // Parse the string into a non-zero u32
        NonZeroU32::from_str(s)
            .map(TimeDelay)
            .map_err(TimeDelayError::from)
    }
}

impl TryFrom<u32> for TimeDelay {
    type Error = TimeDelayError;

    /// Converts a `u32` into a `TimeDelay`
    ///
    /// # Arguments
    ///
    /// * `value` - A `u32` that holds the time delay
    ///
    /// # Returns
    ///
    /// A `TimeDelay` struct with the time delay stored as a `NonZeroU32`.
    ///
    /// # Errors
    ///
    /// Returns a `TimeDelayError` if the input `u32` is not a valid positive integer.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        NonZeroU32::try_from(value)
            .map(TimeDelay)
            .map_err(TimeDelayError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", TimeDelay(NonZeroU32::new(1).unwrap()))]
    #[case("2", TimeDelay(NonZeroU32::new(2).unwrap()))]
    #[case("3", TimeDelay(NonZeroU32::new(3).unwrap()))]
    #[case(r#""3""#, TimeDelay(NonZeroU32::new(3).unwrap()))]
    fn test_from_str(#[case] input: &str, #[case] expected: TimeDelay) {
        assert_eq!(TimeDelay::from_str(input).unwrap(), expected);
    }

    #[rstest]
    #[case(1, TimeDelay(NonZeroU32::new(1).unwrap()))]
    #[case(2, TimeDelay(NonZeroU32::new(2).unwrap()))]
    #[case(3, TimeDelay(NonZeroU32::new(3).unwrap()))]
    fn test_try_from(#[case] input: u32, #[case] expected: TimeDelay) {
        assert_eq!(TimeDelay::try_from(input).unwrap(), expected);
    }

    #[rstest]
    #[case(TimeDelay(NonZeroU32::new(1).unwrap()), "1")]
    #[case(TimeDelay(NonZeroU32::new(2).unwrap()), "2")]
    #[case(TimeDelay(NonZeroU32::new(3).unwrap()), "3")]
    fn test_display(#[case] input: TimeDelay, #[case] expected: &str) {
        assert_eq!(input.to_string(), expected);
    }

    #[rstest]
    #[case("a")]
    #[case("0")]
    fn test_from_str_error(#[case] input: &str) {
        assert!(TimeDelay::from_str(input).is_err());
    }

    #[rstest]
    #[case(0)]
    fn test_try_from_error(#[case] input: u32) {
        assert!(TimeDelay::try_from(input).is_err());
    }
}
