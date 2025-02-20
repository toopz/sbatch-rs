//! This module provides a struct to build sbatch commands.
//!
//! The `Sbatch` struct provides methods to append, overwrite, and discard sbatch options.
//! The `Sbatch` struct also provides a method to add a script to the sbatch command.
//! The `Sbatch` struct provides a method to convert the sbatch command to a string.
//!
//! # Example
//!
//! ```
//! use sbatch_rs::{Sbatch, SbatchOption};
//!
//! let mut sbatch_job = Sbatch::new();
//! let option = SbatchOption::JobName("test".to_string());
//! sbatch_job.append_option(option).unwrap();
//! sbatch_job.with_script("test.sh".to_string());
//! assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch --job-name=test test.sh");
//! ```
use thiserror::Error;

mod sbatch_option_list;

use crate::SbatchOption;
use sbatch_option_list::{SbatchOptionList, SbatchOptionListError};

/// A struct to build sbatch commands.
///
/// The `Sbatch` struct provides methods to append, overwrite, and discard sbatch options.
///
/// # Example
///
/// ```
/// use sbatch_rs::{Sbatch, SbatchOption};
///
/// let mut sbatch_job = Sbatch::new();
/// let option = SbatchOption::JobName("test".to_string());
/// sbatch_job.append_option(option).unwrap();
/// sbatch_job.with_script("test.sh".to_string());
/// assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch --job-name=test test.sh");
/// ```
#[derive(Debug, Clone)]
pub struct Sbatch {
    sbatch_options: SbatchOptionList,
    script: Option<String>,
}

/// Error type for the `Sbatch` struct
///
/// The `SbatchError` enum represents all possible errors that can occur when working with the `Sbatch` struct.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SbatchError {
    #[error("Error when trying to append an existing sbatch option")]
    OptionExists(#[from] SbatchOptionListError),
    #[error("No wrap option or script provided")]
    NoWrapOptionOrScript,
}

impl Sbatch {
    /// Creates a new `Sbatch`.
    ///
    /// # Returns
    ///
    /// A new `Sbatch` struct.
    pub fn new() -> Self {
        Sbatch {
            sbatch_options: SbatchOptionList::new(),
            script: None,
        }
    }

    /// Appends a sbatch option to the list of sbatch options.
    ///
    /// # Arguments
    ///
    /// * `option` - The sbatch option to append
    ///
    /// # Returns
    ///
    /// A mutable reference to the `Sbatch` struct.
    ///
    /// # Errors
    ///
    /// Returns a `SbatchError` if the sbatch option already exists in the list.
    ///
    /// # Example
    ///
    /// ```
    /// use sbatch_rs::{Sbatch, SbatchOption};
    ///
    /// let mut sbatch_job = Sbatch::new();
    ///
    /// let option = SbatchOption::JobName("test".to_string());
    /// sbatch_job.append_option(option).unwrap();
    ///
    /// let option = SbatchOption::JobName("test2".to_string());
    /// let result = sbatch_job.append_option(option);
    /// assert!(result.is_err());
    /// ```
    pub fn append_option(&mut self, option: SbatchOption) -> Result<&mut Self, SbatchError> {
        self.sbatch_options.append(option)?;
        Ok(self)
    }

    /// Overwrites a sbatch option in the list of sbatch options.
    ///
    /// # Arguments
    ///
    /// * `option` - The sbatch option to overwrite
    ///
    /// # Returns
    ///
    /// A mutable reference to the `Sbatch` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use sbatch_rs::{Sbatch, SbatchOption};
    ///
    /// let mut sbatch_job = Sbatch::new();
    ///
    /// let option = SbatchOption::JobName("test".to_string());
    /// sbatch_job.append_option(option).unwrap();
    ///
    /// let option = SbatchOption::JobName("test2".to_string());
    /// sbatch_job.overwrite_option(option);
    /// ```
    pub fn overwrite_option(&mut self, option: SbatchOption) -> &mut Self {
        self.sbatch_options.overwrite(option);
        self
    }

    /// Adds a script to the sbatch command.
    ///
    /// # Arguments
    ///
    /// * `script` - The script to add to the sbatch command
    ///
    /// # Returns
    ///
    /// A mutable reference to the `Sbatch` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use sbatch_rs::Sbatch;
    ///
    /// let mut sbatch_job = Sbatch::new();
    /// sbatch_job.with_script("test.sh".to_string());
    ///
    /// assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch test.sh");
    /// ```
    pub fn with_script(&mut self, script: String) -> &mut Self {
        self.script = Some(script);
        self
    }

    /// Converts the sbatch command to a string.
    ///
    /// # Returns
    ///
    /// A string that represents the sbatch command.
    ///
    /// # Errors
    ///
    /// Returns a `SbatchError` if no wrap option or script is provided.
    ///
    /// # Example
    ///
    /// ```
    /// use sbatch_rs::{Sbatch, SbatchOption};
    ///
    /// let mut sbatch_job = Sbatch::new();
    ///
    /// let option = SbatchOption::JobName("test".to_string());
    /// sbatch_job.append_option(option).unwrap();
    /// sbatch_job.with_script("test.sh".to_string());
    ///
    /// assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch --job-name=test test.sh");
    /// ```
    pub fn to_bash(&self) -> Result<String, SbatchError> {
        let options = self.sbatch_options.to_string();
        let script = &self.script;

        match (options.is_empty(), script) {
            (false, Some(s)) => Ok(format!("sbatch {} {}", options, s)),
            (false, None) => Ok(format!("sbatch {}", options)),
            (true, Some(s)) => Ok(format!("sbatch {}", s)),
            (true, None) => Err(SbatchError::NoWrapOptionOrScript),
        }
    }
}

impl Default for Sbatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_option() {
        let mut sbatch_job = Sbatch::new();
        let option = SbatchOption::JobName("test".to_string());
        sbatch_job.append_option(option).unwrap();
        assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch --job-name=test");
    }

    #[test]
    fn test_overwrite_option() {
        let mut sbatch_job = Sbatch::new();
        let option = SbatchOption::JobName("test".to_string());
        sbatch_job.append_option(option).unwrap();
        let option = SbatchOption::JobName("test2".to_string());
        sbatch_job.overwrite_option(option);
        assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch --job-name=test2");
    }

    #[test]
    fn test_with_script() {
        let mut sbatch_job = Sbatch::new();
        sbatch_job.with_script("test.sh".to_string());
        assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch test.sh");
    }

    #[test]
    fn test_to_string_no_options_or_script() {
        let sbatch_job = Sbatch::new();
        let result = sbatch_job.to_bash();
        assert!(result.is_err());
    }

    #[test]
    fn test_to_string_with_script() {
        let mut sbatch_job = Sbatch::new();
        sbatch_job.with_script("test.sh".to_string());
        assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch test.sh");
    }

    #[test]
    fn test_to_string_with_options() {
        let mut sbatch_job = Sbatch::new();
        let option =
            SbatchOption::Wrap(r#""echo "hello world"""#.to_string());
        sbatch_job.append_option(option).unwrap();
        assert_eq!(
            sbatch_job.to_bash().unwrap(),
            r#"sbatch --wrap="echo "hello world"""#
        );
    }

    #[test]
    fn test_to_string_with_options_and_script() {
        let mut sbatch_job = Sbatch::new();
        let option = SbatchOption::JobName("test".to_string());
        sbatch_job.append_option(option).unwrap();
        sbatch_job.with_script("test.sh".to_string());
        assert_eq!(
            sbatch_job.to_bash().unwrap(),
            r#"sbatch --job-name=test test.sh"#
        );
    }

    #[test]
    fn test_default() {
        let sbatch_job = Sbatch::default();
        let result = sbatch_job.to_bash();
        assert!(result.is_err());
    }
}
