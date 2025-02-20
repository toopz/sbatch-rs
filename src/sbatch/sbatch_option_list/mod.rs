//! This module contains the SbatchOptionList struct which is used to store a list of SbatchOption structs.
//! The SbatchOptionList struct provides methods to append, overwrite, and discard SbatchOption structs.
//! The SbatchOptionList struct also provides a method to get the list of options as a string.

use std::fmt::Display;
use thiserror::Error;

use crate::sbatch_option::SbatchOption;

/// Represents a list of sbatch options.
///
/// The `SbatchOptionList` struct represents a list of sbatch options and provides methods to append, overwrite,
/// and discard sbatch options. The `SbatchOptionList` struct also provides a method to get the list of options as a string.
#[derive(Debug, Clone)]
pub struct SbatchOptionList {
    sbatch_options: Vec<SbatchOption>,
}

/// Error type for the `SbatchOptionList` struct
/// The `SbatchOptionListError` enum represents all possible errors that can occur when working with the `SbatchOptionList` struct.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SbatchOptionListError {
    /// Error when trying to append an existing sbatch option
    #[error("Option already exists for {0}, try overwrite")]
    OptionExists(SbatchOption),
}

impl SbatchOptionList {
    /// Creates a new `SbatchOptionList`.
    pub fn new() -> Self {
        SbatchOptionList {
            sbatch_options: Vec::new(),
        }
    }

    /// Checks if the list of sbatch options contains a specific sbatch option.
    ///
    /// Note: The check is done based on the variant of the sbatch option.
    /// For example, if the list contains `SbatchOption::JobName("test")`, then calling
    /// `contains(SbatchOption::JobName("test"))` will return `true`.
    ///
    /// # Arguments
    ///
    /// * `sbatch_option` - The sbatch option to check for
    ///
    /// # Returns
    ///
    /// A boolean indicating if the sbatch option is in the list.
    pub fn contains(&self, sbatch_option: &SbatchOption) -> bool {
        self.sbatch_options
            .iter()
            .any(|x| sbatch_option.is_same_variant(x))
    }

    /// Discards a specific sbatch option from the list of sbatch options.
    ///
    /// Note: The discard is done based on the variant of the sbatch option.
    /// For example, if the list contains `SbatchOption::JobName("test")`, then calling
    /// `discard(SbatchOption::JobName("test"))` will remove the option from the list.
    ///
    /// # Arguments
    ///
    /// * `sbatch_option` - The sbatch option to discard
    ///
    /// # Returns
    ///
    /// A mutable reference to the `SbatchOptionList` struct.
    pub fn discard(&mut self, sbatch_option: &SbatchOption) -> &mut Self {
        self.sbatch_options
            .retain(|x| !sbatch_option.is_same_variant(x));
        self
    }

    /// Appends a new sbatch option to the list of sbatch options.
    ///
    /// If the sbatch option is already in the list, an error is returned.
    /// For example, if the list contains `SbatchOption::JobName("test")`, then calling
    /// `append(SbatchOption::JobName("new_name"))` will return an error.
    /// Use `overwrite` if you want to replace an existing sbatch option.
    ///
    /// If the sbatch option is not in the list, it will be added to the list.
    ///
    /// # Arguments
    ///
    /// * `sbatch_option` - The sbatch option to append
    ///
    /// # Returns
    ///
    /// A mutable reference to the `SbatchOptionList` struct.
    ///
    /// # Errors
    ///
    /// An error is returned if the sbatch option is already in the list.
    pub fn append(
        &mut self,
        sbatch_option: SbatchOption,
    ) -> Result<&mut Self, SbatchOptionListError> {
        // Check if the option is already in the list
        if self.contains(&sbatch_option) {
            return Err(SbatchOptionListError::OptionExists(sbatch_option));
        }

        // If the option is not in the list, add it
        self.sbatch_options.push(sbatch_option);
        Ok(self)
    }

    /// Overwrites an existing sbatch option in the list of sbatch options.
    ///
    /// If the sbatch option is already in the list, it will be removed and the new sbatch option will be added.
    /// For example, if the list contains `SbatchOption::JobName("test")`, then calling
    /// `overwrite(SbatchOption::JobName("new_name"))` will remove the old option and add the new one.
    ///
    /// # Arguments
    ///
    /// * `sbatch_option` - The sbatch option to overwrite
    ///
    /// # Returns
    ///
    /// A mutable reference to the `SbatchOptionList` struct.
    pub fn overwrite(&mut self, sbatch_option: SbatchOption) -> &mut Self {
        // Check if the option is already in the list
        if self.contains(&sbatch_option) {
            // If the option is in the list, remove it and add the new one
            self.discard(&sbatch_option);
        }

        // Add the new option
        self.sbatch_options.push(sbatch_option);
        self
    }

    /// Returns the list of sbatch options as a string.
    ///
    /// The list of sbatch options is sorted alphabetically and returned as a vector of strings.
    /// For example, if the list contains `SbatchOption::JobName("test")` and `SbatchOption::Partition("general")`,
    /// then calling `get_options()` will return `vec!["--job-name=test", "--partition=general"]`.
    ///
    /// # Returns
    ///
    /// A vector of strings containing the sbatch options.
    pub fn get_options(&self) -> Vec<String> {
        let mut result: Vec<String> = self.sbatch_options.iter().map(|x| x.to_string()).collect();
        result.sort();
        result
    }
}

impl Default for SbatchOptionList {
    /// Creates a new `SbatchOptionList` with default values.
    ///
    /// The default value for `SbatchOptionList` is an empty list of sbatch options.
    fn default() -> Self {
        Self::new()
    }
}

impl Display for SbatchOptionList {
    /// Formats the `SbatchOptionList` for display.
    ///
    /// This method is used to convert the `SbatchOptionList` into a string for display purposes.
    /// The list of sbatch options is sorted alphabetically and returned as a string.
    /// For example, if the list contains `SbatchOption::JobName("test")` and `SbatchOption::Partition("general")`,
    /// then calling `fmt()` will return `--job-name=test --partition=general`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let options = self.get_options();
        write!(f, "{}", options.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_append() {
        // Create a new SbatchOptionList
        let mut sbatch_options = SbatchOptionList::new();

        // Create a new SbatchOption using a string
        let sbatch_option = SbatchOption::from_str("--partition=test");
        if sbatch_option.is_err() {
            panic!("Failed to create SbatchOption: {:?}", sbatch_option.err());
        }
        let sbatch_option = sbatch_option.unwrap();

        // Try to append the SbatchOption to the SbatchOptionList
        let append_result = sbatch_options.append(sbatch_option.clone());
        if append_result.is_err() {
            panic!("Failed to append SbatchOption: {:?}", append_result.err());
        }

        // Check that the SbatchOptionList contains the SbatchOption
        assert_eq!(sbatch_options.sbatch_options.len(), 1);

        // Try to add the same SbatchOption again which should fail
        let append_result = sbatch_options.append(sbatch_option);
        assert!(append_result.is_err());
    }

    #[test]
    fn test_overwrite() {
        let mut sbatch_options = SbatchOptionList::new();
        let sbatch_option = SbatchOption::from_str("--partition=test").unwrap();
        sbatch_options.append(sbatch_option.clone()).unwrap();
        assert_eq!(sbatch_options.sbatch_options.len(), 1);

        let sbatch_option = SbatchOption::from_str("--partition=test").unwrap();
        sbatch_options.overwrite(sbatch_option.clone());
        assert_eq!(sbatch_options.sbatch_options.len(), 1);
    }

    #[test]
    fn test_contains() {
        let mut sbatch_options = SbatchOptionList::new();
        let sbatch_option = SbatchOption::from_str("--partition=test").unwrap();
        sbatch_options.append(sbatch_option.clone()).unwrap();
        assert_eq!(sbatch_options.contains(&sbatch_option), true);
    }

    #[test]
    fn test_discard() {
        let mut sbatch_options = SbatchOptionList::new();
        let sbatch_option = SbatchOption::from_str("--partition=test").unwrap();
        sbatch_options.append(sbatch_option.clone()).unwrap();
        assert_eq!(sbatch_options.sbatch_options.len(), 1);

        sbatch_options.discard(&sbatch_option);
        assert_eq!(sbatch_options.sbatch_options.len(), 0);
    }

    #[test]
    fn test_get_options() {
        let mut sbatch_options = SbatchOptionList::new();
        let sbatch_option = SbatchOption::from_str("--partition=test").unwrap();
        sbatch_options.append(sbatch_option.clone()).unwrap();
        let options = sbatch_options.get_options();
        assert_eq!(options.len(), 1);
        assert_eq!(options[0], "--partition=test");
    }

    #[test]
    fn test_to_string() {
        let mut sbatch_options = SbatchOptionList::new();
        let sbatch_option = SbatchOption::from_str("--partition=test").unwrap();
        sbatch_options.append(sbatch_option.clone()).unwrap();
        let options = sbatch_options.to_string();
        assert_eq!(options, "--partition=test");
    }

    #[test]
    fn test_default() {
        let sbatch_options = SbatchOptionList::default();
        assert_eq!(sbatch_options.sbatch_options.len(), 0);
    }
}
