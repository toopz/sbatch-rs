//! Validation functions for the `DependencyType` enum.

use super::{DependencyType, DependencyTypeError};

/// Helper function to validate a string.
/// This function checks if the string is empty or contains leading or trailing spaces.
///
/// # Arguments
///
/// * `s` - A string to validate.
///
/// # Returns
///
/// This function returns `Ok(())` if the string is valid, otherwise it returns a `DependencyTypeError`.
fn validate_str(s: &str) -> Result<(), DependencyTypeError> {
    if s.trim().is_empty() {
        Err(DependencyTypeError::EmptyDependencyType)
    } else if s != s.trim() {
        Err(DependencyTypeError::LeadingOrTrailingSpaces)
    } else {
        Ok(())
    }
}

impl DependencyType {
    /// Validates the dependency type.
    ///
    /// # Returns
    ///
    /// This function returns `Ok(())` if the dependency type is valid, otherwise it returns a `DependencyTypeError`.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyTypeError` if the dependency type is invalid.
    /// The following are considered invalid:
    /// - An empty string
    /// - A string that contains leading or trailing spaces
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::DependencyType;
    ///
    /// // Valid: mapped to `after:123`
    /// let dependency_type = DependencyType::After("123".to_string());
    /// assert!(dependency_type.validate().is_ok());
    ///
    /// // Valid: mapped to `after:123+10`
    /// let dependency_type = DependencyType::AfterTimeDelay("123".to_string(), "10".to_string());
    /// assert!(dependency_type.validate().is_ok());
    ///
    /// // Invalid: empty string
    /// let dependency_type = DependencyType::After("".to_string());
    /// assert!(dependency_type.validate().is_err());
    ///
    /// // Invalid: leading or trailing spaces
    /// let dependency_type = DependencyType::After(" 123 ".to_string());
    /// assert!(dependency_type.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), DependencyTypeError> {
        match self {
            DependencyType::After(job_id) => validate_str(job_id),
            DependencyType::AfterTimeDelay(job_id, time_delay) => {
                validate_str(job_id)?;
                validate_str(time_delay)
            }
            DependencyType::AfterAny(job_id) => validate_str(job_id),
            DependencyType::AfterBurstBuffer(job_id) => validate_str(job_id),
            DependencyType::AfterCorr(job_id) => validate_str(job_id),
            DependencyType::AfterNotOk(job_id) => validate_str(job_id),
            DependencyType::AfterOk(job_id) => validate_str(job_id),
            DependencyType::Singleton => Ok(()),
        }
    }
}
