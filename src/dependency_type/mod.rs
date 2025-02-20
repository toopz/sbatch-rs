//! This module contains the definition of the DependencyType enum and its associated errors.
//! The DependencyType enum represents the different types of dependencies that can be used in a Slurm job script.
//! The DependencyType enum is a wrapper around the JobId enum and provides a way to parse a string into a valid dependency.
//! The DependencyType enum implements the FromStr trait to allow parsing a string into a DependencyType enum.
//! The DependencyType enum can be one of the following variants:
//! - After(JobId): This job can begin execution after the specified job starts or is cancelled.
//! - AfterTimeDelay(JobId, TimeDelay): This job can begin execution after the specified job starts or is cancelled and 'time' in minutes from job start or cancellation happens.
//! - AfterAny(JobId): This job can begin execution after the specified jobs have terminated.
//! - AfterBurstBuffer(JobId): This job can begin execution after the specified jobs have terminated and any associated burst buffer stage out operations have completed.
//! - AfterCorr(JobId): A task of this job array can begin execution after the corresponding task ID in the specified job has completed successfully (ran to completion with an exit code of zero).
//! - AfterNotOk(JobId): This job can begin execution after the specified jobs have terminated in some failed state (non-zero exit code, node failure, timed out, etc).
//! - AfterOk(JobId): This job can begin execution after the specified jobs have successfully executed (ran to completion with an exit code of zero).
//! - Singleton: This job can begin execution after any previously launched jobs sharing the same job name and user have terminated. In other words, only one job by that name and owned by that user can be running or suspended at any point in time.
use std::fmt::Display;
use std::str::FromStr;

use thiserror::Error;

mod job_id;
mod time_delay;

pub use job_id::{JobId, JobIdError, Variable, VariableError};
pub use time_delay::{TimeDelay, TimeDelayError};

/// Represents a dependency in a Slurm job script.
///
/// The `DependencyType` enum represents the different types of dependencies that can be used in a Slurm job script.
/// The `DependencyType` enum is a wrapper around the `JobId` enum and provides a way to parse a string into a valid dependency.
/// The `DependencyType` enum can be one of the following variants:
/// - After(JobId): This job can begin execution after the specified job starts or is cancelled.
/// - AfterTimeDelay(JobId, TimeDelay): This job can begin execution after the specified job starts or is cancelled and 'time' in minutes from job start or cancellation happens.
/// - AfterAny(JobId): This job can begin execution after the specified jobs have terminated.
/// - AfterBurstBuffer(JobId): This job can begin execution after the specified jobs have terminated and any associated burst buffer stage out operations have completed.
/// - AfterCorr(JobId): A task of this job array can begin execution after the corresponding task ID in the specified job has completed successfully (ran to completion with an exit code of zero).
/// - AfterNotOk(JobId): This job can begin execution after the specified jobs have terminated in some failed state (non-zero exit code, node failure, timed out, etc).
/// - AfterOk(JobId): This job can begin execution after the specified jobs have successfully executed (ran to completion with an exit code of zero).
/// - Singleton: This job can begin execution after any previously launched jobs sharing the same job name and user have terminated. In other words, only one job by that name and owned by that user can be running or suspended at any point in time.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DependencyType {
    After(JobId),
    AfterTimeDelay(JobId, TimeDelay),
    AfterAny(JobId),
    AfterBurstBuffer(JobId),
    AfterCorr(JobId),
    AfterNotOk(JobId),
    AfterOk(JobId),
    Singleton,
}

/// Error type for the `DependencyType` enum
///
/// The `DependencyTypeError` enum represents all possible errors that can occur when working with the `DependencyType` enum.
/// The `DependencyTypeError` enum can be one of the following variants:
/// - InvalidJobId(JobIdError): Failed to parse DependencyType job id from string.
/// - InvalidTimeDelay(TimeDelayError): Failed to parse DependencyType time delay from string.
/// - EmptyString: Empty string is not a valid DependencyType.
/// - SingletonJobIdFound(String): Singleton takes no job id, found a job id.
#[derive(Debug, Error, PartialEq)]
pub enum DependencyTypeError {
    #[error("Failed to parse DependencyType job id from string: {0}")]
    InvalidJobId(JobIdError),
    #[error("Failed to parse DependencyType time delay from string: {0}")]
    InvalidTimeDelay(TimeDelayError),
    #[error("Empty string is not a valid DependencyType")]
    EmptyString,
    #[error("Singleton takes no job id, found: {0}")]
    SingletonJobIdFound(String),
}

impl From<JobIdError> for DependencyTypeError {
    /// Converts a `JobIdError` into a `DependencyTypeError`
    ///
    /// # Arguments
    ///
    /// * `error` - A `JobIdError` enum
    ///
    /// # Returns
    ///
    /// A `DependencyTypeError` enum with the error stored as a `DependencyTypeError::InvalidJobId`
    fn from(error: JobIdError) -> Self {
        DependencyTypeError::InvalidJobId(error)
    }
}

impl From<TimeDelayError> for DependencyTypeError {
    /// Converts a `TimeDelayError` into a `DependencyTypeError`
    ///
    /// # Arguments
    ///
    /// * `error` - A `TimeDelayError` enum
    ///
    /// # Returns
    ///
    /// A `DependencyTypeError` enum with the error stored as a `DependencyTypeError::InvalidTimeDelay`
    fn from(error: TimeDelayError) -> Self {
        DependencyTypeError::InvalidTimeDelay(error)
    }
}

impl DependencyType {
    /// Parses a string into a `DependencyType::After` or `DependencyType::AfterTimeDelay`
    ///
    /// If the input string contains a "+", it will be parsed into a `DependencyType::AfterTimeDelay`.
    /// Otherwise, it will be parsed into a `DependencyType::After`. For example:
    /// - "1" will be parsed into `DependencyType::After(JobId::Number(1))`
    /// - "1+1" will be parsed into `DependencyType::AfterTimeDelay(JobId::Number(1), TimeDelay(1))`
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the job id or job id with time delay
    ///
    /// # Returns
    ///
    /// A `DependencyType::After` or `DependencyType::AfterTimeDelay` enum variant.
    ///
    /// # Errors
    ///
    /// Returns a `DependencyTypeError` if the input string is not a valid job_id or job_id+time_delay.
    pub fn after(job_id: &str) -> Result<Self, DependencyTypeError> {
        // Check if job_id string has a "+" to indicate a time delay
        match job_id.split('+').collect::<Vec<&str>>().as_slice() {
            [job_id, time_delay] => DependencyType::after_time_delay(job_id, time_delay),
            [job_id] => JobId::from_str(job_id)
                .map(DependencyType::After)
                .map_err(DependencyTypeError::from),
            _ => Err(DependencyTypeError::InvalidJobId(JobIdError::ParseError(
                job_id.to_string(),
            ))),
        }
    }

    /// Parses a string into a `DependencyType::AfterTimeDelay`
    ///
    /// This method takes a job_id and time_delay string and parses them into a `DependencyType::AfterTimeDelay`.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the job id
    /// * `time_delay` - A string slice that holds the time delay
    ///
    /// # Returns
    ///
    /// A `DependencyType::AfterTimeDelay` enum variant.
    ///
    /// # Errors
    ///
    /// Returns a `DependencyTypeError` if the input strings are not valid job_id and time_delay.
    pub fn after_time_delay(job_id: &str, time_delay: &str) -> Result<Self, DependencyTypeError> {
        // Try to parse the job_id and time_delay, then create a DependencyType::AfterTimeDelay
        let job_id = JobId::from_str(job_id).map_err(DependencyTypeError::from)?;
        let time_delay = TimeDelay::from_str(time_delay).map_err(DependencyTypeError::from)?;

        // Return the DependencyType::AfterTimeDelay
        Ok(DependencyType::AfterTimeDelay(job_id, time_delay))
    }

    /// Parses a string into a `DependencyType::AfterAny`
    ///
    /// This method takes a job_id string and parses it into a `DependencyType::AfterAny`.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the job id
    ///
    /// # Returns
    ///
    /// A `DependencyType::AfterAny` enum variant.
    ///
    /// # Errors
    ///
    /// Returns a `DependencyTypeError` if the input string is not a valid job_id.
    pub fn after_any(job_id: &str) -> Result<Self, DependencyTypeError> {
        JobId::from_str(job_id)
            .map(DependencyType::AfterAny)
            .map_err(DependencyTypeError::from)
    }

    /// Parses a string into a `DependencyType::AfterBurstBuffer`
    ///
    /// This method takes a job_id string and parses it into a `DependencyType::AfterBurstBuffer`.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the job id
    ///
    /// # Returns
    ///
    /// A `DependencyType::AfterBurstBuffer` enum variant.
    ///
    /// # Errors
    ///
    /// Returns a `DependencyTypeError` if the input string is not a valid job_id.
    pub fn after_burst_buffer(job_id: &str) -> Result<Self, DependencyTypeError> {
        JobId::from_str(job_id)
            .map(DependencyType::AfterBurstBuffer)
            .map_err(DependencyTypeError::from)
    }

    /// Parses a string into a `DependencyType::AfterCorr`
    ///
    /// This method takes a job_id string and parses it into a `DependencyType::AfterCorr`.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the job id
    ///
    /// # Returns
    ///
    /// A `DependencyType::AfterCorr` enum variant.
    ///
    /// # Errors
    ///
    /// Returns a `DependencyTypeError` if the input string is not a valid job_id.
    pub fn after_corr(job_id: &str) -> Result<Self, DependencyTypeError> {
        JobId::from_str(job_id)
            .map(DependencyType::AfterCorr)
            .map_err(DependencyTypeError::from)
    }

    /// Parses a string into a `DependencyType::AfterNotOk`
    ///     
    /// This method takes a job_id string and parses it into a `DependencyType::AfterNotOk`.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the job id
    ///
    /// # Returns
    ///
    /// A `DependencyType::AfterNotOk` enum variant.
    ///
    /// # Errors
    ///
    /// Returns a `DependencyTypeError` if the input string is not a valid job_id.
    pub fn after_not_ok(job_id: &str) -> Result<Self, DependencyTypeError> {
        JobId::from_str(job_id)
            .map(DependencyType::AfterNotOk)
            .map_err(DependencyTypeError::from)
    }

    /// Parses a string into a `DependencyType::AfterOk`
    ///
    /// This method takes a job_id string and parses it into a `DependencyType::AfterOk`.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the job id
    ///
    /// # Returns
    ///
    /// A `DependencyType::AfterOk` enum variant.
    ///
    /// # Errors
    ///
    /// Returns a `DependencyTypeError` if the input string is not a valid job_id.
    pub fn after_ok(job_id: &str) -> Result<Self, DependencyTypeError> {
        JobId::from_str(job_id)
            .map(DependencyType::AfterOk)
            .map_err(DependencyTypeError::from)
    }

    /// Parses a string into a `DependencyType::Singleton`
    ///
    /// This method takes a job_id string and parses it into a `DependencyType::Singleton`.
    /// A job_id is taken as input to ensure a consistent interface with the other DependencyType variants.
    /// The `DependencyType::Singleton` variant does not take a job_id. If a job_id is provided, it will return an error.
    /// All calls to this method should pass an empty string as the job_id.
    /// Alternatively, the `DependencyType::Singleton` variant can be created directly using `DependencyType::Singleton`.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the job id
    ///
    /// # Returns
    ///
    /// A `DependencyType::Singleton` enum variant.
    ///
    /// # Errors
    ///
    /// Returns a `DependencyTypeError` if the input string is not empty.
    pub fn singleton(job_id: &str) -> Result<Self, DependencyTypeError> {
        // Check if the job_id is empty
        match job_id.is_empty() {
            true => Ok(DependencyType::Singleton),
            false => Err(DependencyTypeError::SingletonJobIdFound(job_id.to_string())),
        }
    }
}

impl Display for DependencyType {
    /// Converts a `DependencyType` into a string
    ///
    /// This method converts a `DependencyType` into a string. The string representation of a `DependencyType` is as follows:
    /// - `DependencyType::After(JobId)`: "after:job_id"
    /// - `DependencyType::AfterTimeDelay(JobId, TimeDelay)`: "after:job_id+time"
    /// - `DependencyType::AfterAny(JobId)`: "afterany:job_id"
    /// - `DependencyType::AfterBurstBuffer(JobId)`: "afterburstbuffer:job_id"
    /// - `DependencyType::AfterCorr(JobId)`: "aftercorr:job_id"
    /// - `DependencyType::AfterNotOk(JobId)`: "afternotok:job_id"
    /// - `DependencyType::AfterOk(JobId)`: "afterok:job_id"
    /// - `DependencyType::Singleton`: "singleton"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyType::After(job_id) => write!(f, "after:{}", job_id),
            DependencyType::AfterTimeDelay(job_id, time_delay) => {
                write!(f, "after:{}+{}", job_id, time_delay)
            }
            DependencyType::AfterAny(job_id) => write!(f, "afterany:{}", job_id),
            DependencyType::AfterBurstBuffer(job_id) => write!(f, "afterburstbuffer:{}", job_id),
            DependencyType::AfterCorr(job_id) => write!(f, "aftercorr:{}", job_id),
            DependencyType::AfterNotOk(job_id) => write!(f, "afternotok:{}", job_id),
            DependencyType::AfterOk(job_id) => write!(f, "afterok:{}", job_id),
            DependencyType::Singleton => write!(f, "singleton"),
        }
    }
}

impl FromStr for DependencyType {
    type Err = DependencyTypeError;

    /// Parses a string into a `DependencyType`
    ///
    /// The input string can be one of the following:
    /// - after:job_id: This job can begin execution
    ///   after the specified job starts or is cancelled.
    /// - after:job_id+time: This job can begin execution
    ///   after the specified job starts or is cancelled and 'time' in minutes from job start or cancellation happens.
    /// - afterany:job_id: This job can begin execution
    ///   after the specified jobs have terminated.
    /// - afterburstbuffer:job_id: This job can begin execution
    ///   after the specified jobs have terminated and any associated burst buffer stage out operations have completed.
    /// - aftercorr:job_id: A task of this job array can begin execution
    ///   after the corresponding task ID in the specified job has completed successfully (ran to completion with an exit code of zero).
    /// - afternotok:job_id: This job can begin execution
    ///   after the specified jobs have terminated in some failed state (non-zero exit code, node failure, timed out, etc).
    /// - afterok:job_id: This job can begin execution
    ///   after the specified jobs have successfully executed (ran to completion with an exit code of zero).
    /// - singleton: This job can begin execution
    ///   after any previously launched jobs sharing the same job name and user have terminated.
    ///   In other words, only one job by that name and owned by that user can be running or suspended at any point in time.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the input string
    ///
    /// # Returns
    ///
    /// A `DependencyType` enum variant if the input string can be parsed.
    ///
    /// # Errors
    ///
    /// Returns a `DependencyTypeError` if the input string is not a valid DependencyType.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Trim the input string to remove quotes and spaces
        let value = s.trim_matches('\'').trim_matches('"').trim();

        // Check if the value is empty
        if value.is_empty() {
            return Err(DependencyTypeError::EmptyString);
        }

        // Define the known prefixes
        let prefixes = [
            (
                "after:",
                DependencyType::after as fn(&str) -> Result<Self, DependencyTypeError>,
            ),
            ("afterany:", DependencyType::after_any),
            ("afterburstbuffer:", DependencyType::after_burst_buffer),
            ("aftercorr:", DependencyType::after_corr),
            ("afternotok:", DependencyType::after_not_ok),
            ("afterok:", DependencyType::after_ok),
            ("singleton", DependencyType::singleton),
        ];

        // Iterate over the prefixes to find a match
        for (prefix, parse_fn) in prefixes.iter() {
            if let Some(stripped) = value.strip_prefix(prefix) {
                return parse_fn(stripped);
            }
        }

        // Return an error if no match was found
        Err(DependencyTypeError::InvalidJobId(JobIdError::ParseError(
            value.to_string(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1")]
    #[case("$name")]
    #[case("$name1")]
    #[case("$name_")]
    #[case("$name_1")]
    #[case("${name1}")]
    #[case("${name_}")]
    #[case("${name_1}")]
    fn test_is_ok(#[case] value: &str) {
        // Try to create a Dependency::After
        let dependency = DependencyType::after(value);
        assert!(dependency.is_ok());

        // Try to create a DependencyType::AfterAny
        let dependency = DependencyType::after_any(value);
        assert!(dependency.is_ok());

        // Try to create a DependencyType::AfterBurstBuffer
        let dependency = DependencyType::after_burst_buffer(value);
        assert!(dependency.is_ok());

        // Try to create a DependencyType::AfterCorr
        let dependency = DependencyType::after_corr(value);
        assert!(dependency.is_ok());

        // Try to create a DependencyType::AfterNotOk
        let dependency = DependencyType::after_not_ok(value);
        assert!(dependency.is_ok());

        // Try to create a DependencyType::AfterOk
        let dependency = DependencyType::after_ok(value);
        assert!(dependency.is_ok());

        // Try to create a DependencyType::Singleton
        let dependency = DependencyType::singleton("");
        assert!(dependency.is_ok());

        // Try to create a DependencyType::Singleton using a single string, this is an error
        let dependency = DependencyType::singleton(value);
        assert!(dependency.is_err());
    }

    #[test]
    fn test_after_time_delay_is_ok() {
        // Try to create a DependencyType::AfterTimeDelay
        let dependency = DependencyType::after_time_delay("123", "1");
        assert!(dependency.is_ok());

        // Try to create a DependencyType::AfterTimeDelay using a single string
        let dependency = DependencyType::after("123+1");
        assert!(dependency.is_ok());
    }

    #[rstest]
    #[case("")]
    #[case("asb")]
    fn test_is_error(#[case] value: &str) {
        // Try to create a DependencyType::After
        let dependency = DependencyType::after(value);
        assert!(dependency.is_err());

        // Try to create a DependencyType::AfterAny
        let dependency = DependencyType::after_any(value);
        assert!(dependency.is_err());

        // Try to create a DependencyType::AfterBurstBuffer
        let dependency = DependencyType::after_burst_buffer(value);
        assert!(dependency.is_err());

        // Try to create a DependencyType::AfterCorr
        let dependency = DependencyType::after_corr(value);
        assert!(dependency.is_err());

        // Try to create a DependencyType::AfterNotOk
        let dependency = DependencyType::after_not_ok(value);
        assert!(dependency.is_err());

        // Try to create a DependencyType::AfterOk
        let dependency = DependencyType::after_ok(value);
        assert!(dependency.is_err());
    }

    #[test]
    fn test_after_time_delay_is_error() {
        // Try to create a DependencyType::AfterTimeDelay
        let dependency = DependencyType::after_time_delay("123", "");
        assert!(dependency.is_err());

        // Try to create a DependencyType::AfterTimeDelay
        let dependency = DependencyType::after_time_delay("", "1");
        assert!(dependency.is_err());

        // Try to create a DependencyType::After
        let dependency = DependencyType::after("123+1+4");
        assert!(dependency.is_err());

        let dependency = DependencyType::after("123+");
        assert!(dependency.is_err());
    }

    #[rstest]
    #[case(DependencyType::after("1").unwrap(), "after:1")]
    #[case(DependencyType::after("$variable").unwrap(), "after:${variable}")]
    #[case(DependencyType::after_time_delay("1", "1").unwrap(), "after:1+1")]
    #[case(DependencyType::after_any("1").unwrap(), "afterany:1")]
    #[case(DependencyType::after_burst_buffer("1").unwrap(), "afterburstbuffer:1")]
    #[case(DependencyType::after_corr("1").unwrap(), "aftercorr:1")]
    #[case(DependencyType::after_not_ok("1").unwrap(), "afternotok:1")]
    #[case(DependencyType::after_ok("1").unwrap(), "afterok:1")]
    #[case(DependencyType::Singleton, "singleton")]
    fn test_display(#[case] dependency: DependencyType, #[case] expected: &str) {
        assert_eq!(dependency.to_string(), expected);
    }

    #[rstest]
    #[case("after:1", DependencyType::After(JobId::from_str("1").unwrap()))]
    #[case("after:${variable}", DependencyType::After(JobId::from_str("$variable").unwrap()))]
    #[case(r#""after:${variable}""#, DependencyType::After(JobId::from_str("$variable").unwrap()))]
    #[case("after:1+1", DependencyType::AfterTimeDelay(JobId::from_str("1").unwrap(), TimeDelay::from_str("1").unwrap()))]
    #[case(r#""after:1+1""#, DependencyType::AfterTimeDelay(JobId::from_str("1").unwrap(), TimeDelay::from_str("1").unwrap()))]
    #[case("afterany:1", DependencyType::AfterAny(JobId::from_str("1").unwrap()))]
    #[case("afterburstbuffer:1", DependencyType::AfterBurstBuffer(JobId::from_str("1").unwrap()))]
    #[case("aftercorr:1", DependencyType::AfterCorr(JobId::from_str("1").unwrap()))]
    #[case("afternotok:1", DependencyType::AfterNotOk(JobId::from_str("1").unwrap()))]
    #[case("afterok:1", DependencyType::AfterOk(JobId::from_str("1").unwrap()))]
    #[case("singleton", DependencyType::Singleton)]
    fn test_from_str(#[case] value: &str, #[case] expected: DependencyType) {
        let dependency = DependencyType::from_str(value).unwrap();
        assert_eq!(dependency, expected);
    }

    #[rstest]
    #[case("")]
    #[case("asb")]
    fn test_from_str_is_error(#[case] value: &str) {
        let dependency = DependencyType::from_str(value);
        assert!(dependency.is_err());
    }
}
