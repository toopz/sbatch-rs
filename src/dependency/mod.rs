//! This module contains the `Dependency` enum and related types.

use std::collections::BTreeSet;
use thiserror::Error;

mod dependency_type;
pub use dependency_type::{DependencyType, DependencyTypeError};

/// Sbatch dependency representation
/// 
/// Represents the different types of dependencies that can be used in a Slurm job script.
/// See <https://slurm.schedmd.com/sbatch.html> for more information.
///
/// - `And(Vec<DependencyType>)`: The job can start after all of the specified dependencies have been met.
/// - `Or(Vec<DependencyType>)`: The job can start after any of the specified dependencies have been met.
///
/// # Examples
///
/// ```
/// use sbatch_rs::{Dependency, DependencyType};
///
/// // Create a new `And` dependency
/// let dependency = Dependency::new_and()
///     .push(DependencyType::After("123".to_string())).unwrap() // Add an `After` dependency
///     .push_after_time_delay("456", "10").unwrap() // Add an `AfterTimeDelay` dependency
///     .build().unwrap(); // Build the dependency string
///
/// // Check that the dependency string is correct
/// assert_eq!(dependency, "after:123,after:456+10");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Dependency {
    And(Vec<DependencyType>),
    Or(Vec<DependencyType>),
}

/// Represents an error that can occur when working with the `Dependency` enum.
/// This error is used to indicate that a `Dependency` value is invalid.
///
/// - `NoDependencies`: Indicates that no dependencies were provided.
/// - `DependencyTypeError`: Indicates that a `DependencyType` value is invalid.
#[derive(Debug, Error)]
pub enum DependencyError {
    #[error("No dependencies provided")]
    NoDependencies,
    #[error("Dependency type error: {0}")]
    DependencyTypeError(#[from] dependency_type::DependencyTypeError),
}

// Helper functions for the `Dependency` enum
impl Dependency {
    // Helper function to get the separator for the dependency string.
    fn separator(&self) -> &str {
        match self {
            Dependency::And(_) => ",",
            Dependency::Or(_) => "?",
        }
    }

    // Helper function to get the dependencies vector.
    fn dependencies(&self) -> &Vec<DependencyType> {
        match &self {
            Dependency::And(dependencies) => dependencies,
            Dependency::Or(dependencies) => dependencies,
        }
    }
}

// Interface functions for the `Dependency` enum
impl Dependency {
    /// Create a new `And` dependency.
    ///
    /// # Returns
    ///
    /// This function returns a new `Dependency` enum with an `And` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// // Create a new `And` dependency
    /// let dependency = Dependency::new_and();
    /// ```
    pub fn new_and() -> Self {
        Dependency::And(Vec::new())
    }

    /// Create a new `Or` dependency.
    ///
    /// # Returns
    ///
    /// This function returns a new `Dependency` enum with an `Or` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// // Create a new `Or` dependency
    /// let dependency = Dependency::new_or();
    /// ```
    pub fn new_or() -> Self {
        Dependency::Or(Vec::new())
    }

    /// Add a dependency to the `Dependency` enum.
    ///
    /// # Arguments
    ///
    /// * `dependency` - A `DependencyType` value to add to the `Dependency` enum.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Dependency` enum.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyError` if the dependency is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::{Dependency, DependencyType};
    ///
    /// // Create a new `And` dependency
    /// let mut dependency = Dependency::new_and();
    ///
    /// // Add an `After` dependency using the enum variant
    /// dependency.push(DependencyType::After("123".to_string())).unwrap();
    ///
    /// // Build the dependency string
    /// let dependency_str = dependency.build().unwrap();
    /// assert_eq!(dependency_str, "after:123");
    /// ```
    pub fn push(&mut self, dependency: DependencyType) -> Result<&mut Self, DependencyError> {
        // Validate the dependency
        dependency.validate()?;

        // Add the dependency to the vector
        match self {
            Dependency::And(dependencies) => dependencies.push(dependency),
            Dependency::Or(dependencies) => dependencies.push(dependency),
        }
        Ok(self)
    }

    /// Add an `After` dependency to the `Dependency` enum.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The job ID to add as a dependency.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Dependency` enum.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyError` if the dependency is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// let dependency = Dependency::new_and()
    ///     .push_after("123").unwrap()
    ///     .build().unwrap();
    /// assert_eq!(dependency, "after:123");
    ///
    /// ```
    pub fn push_after(&mut self, job_id: impl ToString) -> Result<&mut Self, DependencyError> {
        self.push(DependencyType::After(job_id.to_string()))
    }

    /// Add an `AfterTimeDelay` dependency to the `Dependency` enum.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The job ID to add as a dependency.
    /// * `time_delay` - The time delay to add as a dependency.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Dependency` enum.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyError` if the dependency is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// let dependency = Dependency::new_and()
    ///     .push_after_time_delay("123", "10").unwrap()
    ///     .build().unwrap();
    /// assert_eq!(dependency, "after:123+10");
    /// ```
    pub fn push_after_time_delay(
        &mut self,
        job_id: impl ToString,
        time_delay: impl ToString,
    ) -> Result<&mut Self, DependencyError> {
        self.push(DependencyType::AfterTimeDelay(
            job_id.to_string(),
            time_delay.to_string(),
        ))
    }

    /// Add an `AfterAny` dependency to the `Dependency` enum.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The job ID to add as a dependency.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Dependency` enum.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyError` if the dependency is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// let dependency = Dependency::new_and()
    ///     .push_after_any("123").unwrap()
    ///     .build().unwrap();
    /// assert_eq!(dependency, "afterany:123");
    /// ```
    pub fn push_after_any(&mut self, job_id: impl ToString) -> Result<&mut Self, DependencyError> {
        self.push(DependencyType::AfterAny(job_id.to_string()))
    }

    /// Add an `AfterBurstBuffer` dependency to the `Dependency` enum.
    ///     
    /// # Arguments
    ///
    /// * `job_id` - The job ID to add as a dependency.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Dependency` enum.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyError` if the dependency is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// let dependency = Dependency::new_and()
    ///     .push_after_burst_buffer("123").unwrap()
    ///     .build().unwrap();
    /// assert_eq!(dependency, "afterburstbuffer:123");
    ///
    /// ```
    pub fn push_after_burst_buffer(
        &mut self,
        job_id: impl ToString,
    ) -> Result<&mut Self, DependencyError> {
        self.push(DependencyType::AfterBurstBuffer(job_id.to_string()))
    }

    /// Add an `AfterCorr` dependency to the `Dependency` enum.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The job ID to add as a dependency.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Dependency` enum.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyError` if the dependency is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// let dependency = Dependency::new_and()
    ///     .push_after_corr("123").unwrap()
    ///     .build().unwrap();
    /// assert_eq!(dependency, "aftercorr:123");
    /// ```
    pub fn push_after_corr(&mut self, job_id: impl ToString) -> Result<&mut Self, DependencyError> {
        self.push(DependencyType::AfterCorr(job_id.to_string()))
    }

    /// Add an `AfterNotOk` dependency to the `Dependency` enum.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The job ID to add as a dependency.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Dependency` enum.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyError` if the dependency is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// let dependency = Dependency::new_and()
    ///     .push_after_not_ok("123").unwrap()
    ///     .build().unwrap();
    /// assert_eq!(dependency, "afternotok:123");
    /// ```
    pub fn push_after_not_ok(
        &mut self,
        job_id: impl ToString,
    ) -> Result<&mut Self, DependencyError> {
        self.push(DependencyType::AfterNotOk(job_id.to_string()))
    }

    /// Add an `AfterOk` dependency to the `Dependency` enum.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The job ID to add as a dependency.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Dependency` enum.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyError` if the dependency is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// let dependency = Dependency::new_and()
    ///     .push_after_ok("123").unwrap()
    ///     .build().unwrap();
    /// assert_eq!(dependency, "afterok:123");
    /// ```
    pub fn push_after_ok(&mut self, job_id: &str) -> Result<&mut Self, DependencyError> {
        self.push(DependencyType::AfterOk(job_id.to_string()))
    }

    /// Add a `Singleton` dependency to the `Dependency` enum.
    ///
    /// # Returns
    ///
    /// This function returns a mutable reference to the `Dependency` enum.
    ///
    /// # Errors
    ///
    /// Note that the `Singleton` dependency type does not require any arguments.
    /// Currently, this function does not return any errors but may be updated in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::Dependency;
    ///
    /// let dependency = Dependency::new_and()
    ///     .push_singleton().unwrap()
    ///     .build().unwrap();
    /// assert_eq!(dependency, "singleton");
    /// ```
    pub fn push_singleton(&mut self) -> Result<&mut Self, DependencyError> {
        self.push(DependencyType::Singleton)
    }

    /// Build the dependency string.
    ///
    /// # Returns
    ///
    /// This function returns a `String` containing the dependency string.
    ///
    /// # Errors
    ///
    /// This function returns a `DependencyError` if the dependency is invalid.
    /// The `NoDependencies` error is returned if no dependencies were provided.
    /// The `DependencyTypeError` error is returned if a dependency is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::{Dependency, DependencyType};
    ///
    /// // Create a new `And` dependency
    /// let mut dependency = Dependency::new_and();
    ///
    /// // Add an `After` dependency using the enum variant
    /// dependency.push(DependencyType::After("123".to_string())).unwrap();
    ///
    /// // Add a `AfterTimeDelay` dependency using the helper function
    /// dependency.push_after_time_delay("456", "10").unwrap();
    ///
    /// // Build the dependency string
    /// let dependency_str = dependency.build().unwrap();
    /// assert_eq!(dependency_str, "after:123,after:456+10");
    /// ```
    pub fn build(&self) -> Result<String, DependencyError> {
        // Check if there are any dependencies
        if self.dependencies().is_empty() {
            return Err(DependencyError::NoDependencies);
        }

        // Validate the dependencies
        for dependency in self.dependencies() {
            dependency.validate()?;
        }

        // Convert the dependencies to a single string
        Ok(self
            .dependencies()
            .iter()
            .map(|d| d.to_string())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .join(self.separator()))
    }
}
