//! SbatchDependency module for parsing and formatting dependencies in Slurm sbatch options
//!
//! The `SbatchDependency` struct represents a list of dependencies in a Slurm sbatch option.
//! The `SbatchDependency` struct is a wrapper around the `Dependency` struct and provides a way to parse
//! a string into a valid dependency list. The `SbatchDependency` struct implements the `FromStr` trait to
//! allow parsing a string into a `SbatchDependency` struct.
use std::collections::BTreeSet;
use std::fmt::Display;
use std::str::FromStr;

use thiserror::Error;

use crate::DependencyType;

/// Represents the separator between dependencies
///
/// In Slurm's sbatch options, dependencies can be separated by a comma (`,`) or a question mark (`?`).
/// The `And` variant represents a comma separator, while the `Or` variant represents a question mark separator.
/// By default, the separator is `And` in Slurm. Note that mixed separators are not allowed according to the Slurm
/// documentation, so the `DependencySeparator` enum enforces this rule.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DependencySeparator {
    #[default]
    And,
    Or,
}

/// Represents a list of dependencies in a Slurm sbatch option
///
/// The `SbatchDependency` struct represents a list of dependencies in a Slurm sbatch option. It is a wrapper around
/// the `Dependency` struct and provides a way to parse a string into a valid dependency list. The `SbatchDependency`
/// struct implements the `FromStr` trait to allow parsing a string into a `SbatchDependency` struct.
///
/// Duplicate dependencies are not allowed in a `SbatchDependency`. If a duplicate dependency is added, it will be
/// ignored and not included in the final list of dependencies.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SbatchDependency {
    dependencies: BTreeSet<DependencyType>,
    separator: DependencySeparator,
}

/// Errors that can occur when parsing a `SbatchDependency` from a string
///
/// The `SbatchDependencyError` enum represents all possible errors that can occur when parsing a `SbatchDependency`
/// from a string.
#[derive(Debug, Error, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum SbatchDependencyError {
    /// Mixed separators are not allowed. The separator must be either a comma or a question mark.
    #[error("Mixed separators are not allowed: {0}")]
    MixedSeparators(String),
    /// Failed to parse a `Dependency` from a string, likely due to an invalid format.
    #[error("Failed to parse SbatchDependency from string: {0}")]
    ParseError(String),
}

impl SbatchDependency {
    /// Creates a new `SbatchDependency`
    ///
    /// This method creates a new `SbatchDependency` with an empty list of dependencies and the default separator.
    /// The default separator is a comma (`,`) in Slurm.
    pub fn new() -> Self {
        SbatchDependency::default()
    }

    /// Sets a dependency in the list
    ///
    /// This method sets a dependency in the list of dependencies. If the dependency is already in the list,
    /// it will be ignored and not included in the final list of dependencies.
    ///
    /// # Arguments
    ///
    /// * `dependency` - A `DependencyType` to add to the list
    pub fn set_dependency(&mut self, dependency: DependencyType) {
        self.dependencies.insert(dependency);
    }

    /// Sets the separator between dependencies
    ///
    /// This method sets the separator between dependencies. The separator can be either a comma (`,`) or a question mark (`?`).
    /// By default, the separator is a comma in Slurm.
    ///
    /// # Arguments
    ///
    /// * `separator` - A `DependencySeparator` that represents the separator between dependencies
    pub fn set_separator(&mut self, separator: DependencySeparator) {
        self.separator = separator;
    }
}

/// Helper function to determine the separator between dependencies
///
/// This function determines the separator between dependencies in a string. The separator can be either a comma
/// (`,`) or a question mark (`?`). If the string contains both separators, an error is returned.
///
/// # Arguments
///
/// * `s` - A string slice that holds the input string
///
/// # Returns
///
/// A `DependencySeparator` that represents the separator between dependencies.
///
/// # Errors
///
/// Returns a `SbatchDependencyError` if the string contains both separators.
fn determine_separator(s: &str) -> Result<DependencySeparator, SbatchDependencyError> {
    match (s.contains(","), s.contains("?")) {
        (true, false) => Ok(DependencySeparator::And),
        (false, true) => Ok(DependencySeparator::Or),
        (false, false) => Ok(DependencySeparator::And), // default value
        (true, true) => Err(SbatchDependencyError::MixedSeparators(s.to_string())),
    }
}

/// Helper function to split a string by a separator
///
/// This function splits a string by a separator and returns an iterator over the resulting substrings.
/// The separator can be either a comma (`,`) or a question mark (`?`).
///
/// # Arguments
///
/// * `s` - A string slice that holds the input string
/// * `separator` - A `DependencySeparator` that represents the separator between dependencies
///
/// # Returns
///
/// An iterator over the resulting substrings after splitting the input string by the separator.
fn split_by_separator<'a>(
    s: &'a str,
    separator: &DependencySeparator,
) -> impl Iterator<Item = &'a str> {
    match separator {
        DependencySeparator::And => s.trim_matches('"').split(','),
        DependencySeparator::Or => s.trim_matches('"').split('?'),
    }
}

/// Helper function to split a dependency by a colon
///
/// In Slurm, dependecies can be written as "after:1:2", where "after" is the dependency type and "1:2" are the
/// job ids. This is syntactic sugar for "after:1,after:2". This function splits the dependency by colon
/// and returns a vector of strings, where each string is a dependency with the dependency type and a single job id.
///
/// # Arguments
///
/// * `s` - A string slice that holds the input dependency
///
/// # Returns
///
/// A vector of strings, where each string is a dependency with the dependency type and a single job id.
///
/// # Errors
///
/// Returns a `SbatchDependencyError` if the dependency type is not recognized.
/// Recognized dependency types are: "after", "afterany", "afterok", "afternotok", "aftercorr", "afterburstbuffer",
/// and "singleton".
fn split_dependency_by_colon(s: &str) -> Result<Vec<String>, SbatchDependencyError> {
    // Split the dependency by colon to get the job name and ids
    match s
        .trim_matches('"')
        .split(':')
        .collect::<Vec<&str>>()
        .as_slice()
    {
        [dep_type @ ("after" | "afterany" | "afterok" | "afternotok" | "aftercorr"
        | "afterburstbuffer"), dep_args @ ..] => {
            let result: Vec<String> = dep_args
                .iter()
                .map(|s| format!("{}:{}", dep_type, s))
                .collect();
            Ok(result)
        }
        ["singleton"] => Ok(vec!["singleton".to_string()]),
        _ => Err(SbatchDependencyError::ParseError(s.to_string())),
    }
}

impl FromStr for SbatchDependency {
    type Err = SbatchDependencyError;

    /// Parses a string into a `SbatchDependency`
    ///
    /// This method is used to create a `SbatchDependency` from a string by validating the input.
    /// The input string should contain a list of dependencies separated by a comma (`,`) or a question mark (`?`).
    /// Each dependency can be written as "after:1", "after:1:2", or "singleton".
    /// For example: "after:1,after:2" will be parsed as two dependencies: "after:1" and "after:2".
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the input string
    ///
    /// # Returns
    ///
    /// A `SbatchDependency` struct with the parsed dependencies.
    ///
    /// # Errors
    ///
    /// Returns a `SbatchDependencyError` if the input string is empty, contains invalid characters, or contains
    /// mixed separators.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Create a new SbatchDependency
        let mut dependency_list = SbatchDependency::new();

        // Determine the separator
        let separator = determine_separator(s)?;
        dependency_list.set_separator(separator.clone());

        // Collect all parsed dependencies, checking for errors
        let dependencies: Result<Vec<DependencyType>, SbatchDependencyError> =
            split_by_separator(s.trim_matches('"'), &separator)
                .flat_map(|dependency| split_dependency_by_colon(dependency).into_iter())
                .flatten()
                .map(|dep| {
                    DependencyType::from_str(&dep)
                        .map_err(|_| SbatchDependencyError::ParseError(dep))
                })
                .collect();

        // If there are any errors, return the first error encountered
        let dependencies = dependencies?;

        // If all dependencies are parsed successfully, add them to the list
        for dependency in dependencies {
            dependency_list.set_dependency(dependency);
        }

        Ok(dependency_list)
    }
}

impl Display for SbatchDependency {
    /// Formats the dependencies as a string
    ///
    /// This method formats the dependencies as a string. The dependencies are sorted and joined by the separator.
    /// The separator can be either a comma (`,`) or a question mark (`?`).
    /// Note that the shorthand syntax for dependencies is not used in the output string as this can cause
    /// confusion when reading the dependencies. Instead, the full syntax is used for each dependency.
    /// For example: "after:1,after:2" instead of "after:1:2".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Convert the dependencies to a vector of strings
        let mut result = self
            .dependencies
            .iter()
            .map(|dependency| dependency.to_string())
            .collect::<Vec<String>>();

        // Sort for deterministic output
        result.sort();

        // Join the dependencies with the separator
        let s = match self.separator {
            DependencySeparator::And => result.join(","),
            DependencySeparator::Or => result.join("?"),
        };

        write!(f, r#""{}""#, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(DependencySeparator::And, r#""after:1""#)]
    #[case(DependencySeparator::Or, r#""after:1""#)]
    fn test_single_dependency(#[case] separator: DependencySeparator, #[case] expected: &str) {
        let mut dependencies = SbatchDependency::new();
        dependencies.set_dependency(DependencyType::after("1").unwrap());
        dependencies.set_separator(separator);

        assert_eq!(dependencies.to_string(), expected);
    }

    #[test]
    fn test_and_dependencies() {
        let mut dependencies = SbatchDependency::new();
        dependencies.set_dependency(DependencyType::after("1").unwrap());
        dependencies.set_dependency(DependencyType::after("2").unwrap());
        dependencies.set_separator(DependencySeparator::And);

        assert_eq!(dependencies.to_string(), r#""after:1,after:2""#);
    }

    #[test]
    fn test_or_dependencies() {
        let mut dependencies = SbatchDependency::new();
        dependencies.set_dependency(DependencyType::after("1").unwrap());
        dependencies.set_dependency(DependencyType::after("2").unwrap());
        dependencies.set_separator(DependencySeparator::Or);

        assert_eq!(dependencies.to_string(), r#""after:1?after:2""#);
    }

    #[rstest]
    #[case("after:1", DependencySeparator::And)]
    #[case("after:1,after:2", DependencySeparator::And)]
    #[case("after:1?after:2", DependencySeparator::Or)]
    #[case(r#""after:1?after:2""#, DependencySeparator::Or)]
    fn test_determine_separator(#[case] input_value: &str, #[case] expected: DependencySeparator) {
        let result = determine_separator(input_value);
        if result.is_err() {
            panic!("Unexpected error: {}", result.err().unwrap());
        }
        let result = result.unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_determine_separator_error() {
        let result = determine_separator("after:1?after:2,after:3");
        assert!(result.is_err());
    }

    #[rstest]
    #[case("after:1", DependencySeparator::And, vec!["after:1"])]
    #[case("after:1:2", DependencySeparator::And, vec!["after:1:2"])]
    #[case(r#""after:1:2""#, DependencySeparator::And, vec![r"after:1:2"])]
    #[case("after:1,after:2", DependencySeparator::And, vec!["after:1", "after:2"])]
    #[case("after:1?after:2", DependencySeparator::Or, vec!["after:1", "after:2"])]
    fn test_split_by_separator(
        #[case] input_value: &str,
        #[case] separator: DependencySeparator,
        #[case] expected: Vec<&str>,
    ) {
        let result: Vec<&str> = split_by_separator(input_value, &separator).collect();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("after:1", vec!["after:1"])]
    #[case("after:1:2", vec!["after:1", "after:2"])]
    #[case("after:1:2:3", vec!["after:1", "after:2", "after:3"])]
    #[case("afterok:1", vec!["afterok:1"])]
    #[case("afterok:1:2", vec!["afterok:1", "afterok:2"])]
    #[case("afterok:1:2:3", vec!["afterok:1", "afterok:2", "afterok:3"])]
    #[case(r#""afterok:1:2:3""#, vec!["afterok:1", "afterok:2", "afterok:3"])]
    #[case("afternotok:1", vec!["afternotok:1"])]
    #[case("afternotok:1:2", vec!["afternotok:1", "afternotok:2"])]
    #[case("afternotok:1:2:3", vec!["afternotok:1", "afternotok:2", "afternotok:3"])]
    #[case("aftercorr:1", vec!["aftercorr:1"])]
    #[case("aftercorr:1:2", vec!["aftercorr:1", "aftercorr:2"])]
    #[case("aftercorr:1:2:3", vec!["aftercorr:1", "aftercorr:2", "aftercorr:3"])]
    #[case("afterburstbuffer:1", vec!["afterburstbuffer:1"])]
    #[case("afterburstbuffer:1:2", vec!["afterburstbuffer:1", "afterburstbuffer:2"])]
    #[case("afterburstbuffer:1:2:3", vec!["afterburstbuffer:1", "afterburstbuffer:2", "afterburstbuffer:3"])]
    #[case("singleton", vec!["singleton"])]
    fn test_split_dependency_by_colon(#[case] input_value: &str, #[case] expected: Vec<&str>) {
        let result = split_dependency_by_colon(input_value);
        if result.is_err() {
            panic!("Unexpected error: {}", result.err().unwrap());
        }
        let result = result.unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_dependency_by_colon_error() {
        let result = split_dependency_by_colon("not_valid:1:2:3:4");
        assert!(result.is_err());
    }

    #[rstest]
    #[case("after:1", 1)]
    #[case("after:1,after:2", 2)]
    #[case("after:1?after:2", 2)]
    #[case("after:1:2", 2)]
    #[case("after:1:2:3", 3)]
    #[case("afterok:1:2:3", 3)]
    #[case(r#""afterok:1:2:3""#, 3)]
    #[case("singleton", 1)]
    fn test_parse(#[case] input_value: &str, #[case] length: u32) {
        let result = SbatchDependency::from_str(input_value);
        if result.is_err() {
            panic!("Unexpected error: {}", result.err().unwrap());
        }
        let result = result.unwrap();
        assert_eq!(result.dependencies.len() as u32, length);
    }
}
