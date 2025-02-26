//! This module provides a builder for the `sbatch` command in Slurm.

use std::collections::BTreeSet;
use thiserror::Error;

use crate::{SbatchOption, SbatchOptionError};

/// sbatch command builder
///
/// # Examples
///
/// ```
/// use sbatch_rs::{Sbatch, SbatchOption};
///
/// // Create a new `Sbatch` instance
/// let sbatch = Sbatch::new()
///     .add_option(SbatchOption::JobName("test".to_string())).unwrap()
///     .add_option(SbatchOption::Output("test.out".to_string())).unwrap()
///     .add_option(SbatchOption::Error("test.err".to_string())).unwrap()
///     .set_script("test.sh".to_string()).unwrap()
///     .build();
///
/// // Verify that the `sbatch` command was built properly
/// assert!(sbatch.is_ok());
/// assert_eq!(sbatch.unwrap(), "sbatch --error=test.err --job-name=test --output=test.out test.sh");
/// ```
#[derive(Debug, Clone)]
pub struct Sbatch {
    sbatch_options: Option<BTreeSet<SbatchOption>>,
    script: Option<String>,
}

/// The `SbatchError` enum represents an error that can occur when building an `sbatch` command.
///
/// Errors include:
/// - No options or script provided
/// - Script is empty
/// - Sbatch option error
#[derive(Debug, Error)]
pub enum SbatchError {
    #[error("No sbatch options or script provided")]
    NoOptionsOrScript,
    #[error("Script is empty")]
    ScriptEmpty,
    #[error("Sbatch option error: {0}")]
    SbatchOptionError(#[from] SbatchOptionError),
    #[error("Execution failed: {0}")]
    SbatchExecutionError(String),
}

impl Sbatch {
    /// Creates a new `Sbatch` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Sbatch;
    ///
    /// // Create a new `Sbatch` instance
    /// let sbatch = Sbatch::new();
    /// ```
    pub fn new() -> Self {
        Sbatch {
            sbatch_options: None,
            script: None,
        }
    }

    /// Adds an `SbatchOption` to the `Sbatch` instance.
    ///
    /// # Arguments
    ///
    /// * `option` - An `SbatchOption` to add to the `Sbatch` instance.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Sbatch` instance.
    ///
    /// # Errors
    ///
    /// This function returns a `SbatchError` if the `SbatchOption` is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::{Sbatch, SbatchOption};
    ///
    /// // Create a new `Sbatch` instance
    /// let sbatch = Sbatch::new()
    ///     .add_option(SbatchOption::JobName("test".to_string())).unwrap()
    ///     .add_option(SbatchOption::Output("test.out".to_string())).unwrap()
    ///     .add_option(SbatchOption::Error("test.err".to_string())).unwrap()
    ///     .add_option(SbatchOption::Wrap("test".to_string())).unwrap()
    ///     .build();
    ///
    /// // Verify that the `sbatch` command was built properly
    /// assert!(sbatch.is_ok());
    /// assert_eq!(sbatch.unwrap(), "sbatch --error=test.err --job-name=test --output=test.out --wrap=\"test\"");
    /// ```
    pub fn add_option(&mut self, option: SbatchOption) -> Result<&mut Self, SbatchError> {
        // Validate the option
        option.validate()?;

        // Add the option to the set
        self.sbatch_options
            .get_or_insert_with(BTreeSet::new)
            .insert(option);
        Ok(self)
    }

    /// Sets the script for the `Sbatch` instance.
    ///
    /// # Arguments
    ///
    /// * `script` - A string representing the script to run.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Sbatch` instance.
    ///
    /// # Errors
    ///
    /// This function returns a `SbatchError` if the script is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Sbatch;
    ///
    /// // Create a new `Sbatch` instance
    /// let sbatch = Sbatch::new()
    ///     .set_script("test.sh".to_string()).unwrap()
    ///     .build();
    ///
    /// // Verify that the `sbatch` command was built properly
    /// assert!(sbatch.is_ok());
    /// assert_eq!(sbatch.unwrap(), "sbatch test.sh");
    /// ```
    pub fn set_script(&mut self, script: String) -> Result<&mut Self, SbatchError> {
        let script = script.trim().to_string();
        if script.is_empty() {
            Err(SbatchError::ScriptEmpty)
        } else {
            self.script = Some(script);
            Ok(self)
        }
    }

    /// Builds the `sbatch` command.
    ///
    /// # Returns
    ///
    /// This function returns a string representing the `sbatch` command.
    ///
    /// # Errors
    ///
    /// This function returns a `SbatchError` if no options or script are provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::{Sbatch, SbatchOption};
    ///
    /// // Create a new `Sbatch` instance
    /// let sbatch = Sbatch::new()
    ///     .add_option(SbatchOption::JobName("test".to_string())).unwrap()
    ///     .add_option(SbatchOption::Output("test.out".to_string())).unwrap()
    ///     .add_option(SbatchOption::Error("test.err".to_string())).unwrap()
    ///     .set_script("test.sh".to_string()).unwrap()
    ///     .build();
    ///     
    /// // Verify that the `sbatch` command was built properly
    /// assert!(sbatch.is_ok());
    /// assert_eq!(sbatch.unwrap(), "sbatch --error=test.err --job-name=test --output=test.out test.sh");
    pub fn build(&self) -> Result<String, SbatchError> {
        // Convert the sbatch options to a space-joined string
        let options: Option<String> = self.sbatch_options.as_ref().map(|options| {
            options
                .iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        });

        // Combine the options and script
        match (options, &self.script) {
            (Some(o), Some(s)) => Ok(format!("sbatch {o} {s}")),
            (Some(o), None) => Ok(format!("sbatch {o}")),
            (None, Some(s)) => Ok(format!("sbatch {s}")),
            (None, None) => Err(SbatchError::NoOptionsOrScript),
        }
    }
}

impl Default for Sbatch {
    /// Creates a default `Sbatch` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Sbatch;
    ///
    /// // Create a default `Sbatch` instance
    /// let sbatch = Sbatch::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
