//! This module contains the `DependencyType` enum and related types.

use thiserror::Error;

mod display;
mod validate;

/// Sbatch dependency type
/// 
/// The `DependencyType` enum is used to represent the different types of dependencies that can be used in a Slurm job script.
/// For more details on the different types of dependencies, see the Slurm documentation: <https://slurm.schedmd.com/sbatch.html>.
///
/// - `After(String)`: The job can start after the specified job start
/// - `AfterTimeDelay(String, String)`: The job can start after the specified job starts with a time delay.
/// - `AfterAny(String)`: This job can begin execution after the specified jobs have terminated. This is the default dependency type. (Any exit status).
/// - `AfterBurstBuffer(String)`: This job can begin execution after the specified jobs have terminated and any associated burst buffer stage out operations have completed.
/// - `AfterCorr(String)`: A task of this job array can begin execution after the corresponding task ID in the specified job has completed successfully (ran to completion with an exit code of zero).
/// - `AfterNotOk(String)`: This job can begin execution after the specified jobs have terminated in some failed state (non-zero exit code, node failure, timed out, etc).
/// - `AfterOk(String)`: This job can begin execution after the specified jobs have successfully executed (ran to completion with an exit code of zero).
/// - `Singleton`: This job can begin execution after any previously launched jobs sharing the same job name and user have terminated. In other words, only one job by that name and owned by that user can be running or suspended at any point in time.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DependencyType {
    /// Maps to the `after:<job_id>` dependency type
    After(String),
    /// Maps to the `after:<job_id>+<time_delay>` dependency type
    AfterTimeDelay(String, String),
    /// Maps to the `afterany:<job_id>` dependency type
    AfterAny(String),
    /// Maps to the `afterburstbuffer:<job_id>` dependency type
    AfterBurstBuffer(String),
    /// Maps to the `aftercorr:<job_id>` dependency type
    AfterCorr(String),
    /// Maps to the `afternotok:<job_id>` dependency type
    AfterNotOk(String),
    /// Maps to the `afterok:<job_id>` dependency type
    AfterOk(String),
    /// Maps to the `singleton` dependency type
    Singleton,
}

/// Represents an error that can occur when validating a `DependencyType` value.
/// This error is used to indicate that a `DependencyType` value is invalid.
#[derive(Debug, Error)]
pub enum DependencyTypeError {
    /// Indicates that the `DependencyType` value is invalid because it is empty.
    #[error("Dependency type is empty")]
    EmptyDependencyType,
    /// Indicates that the `DependencyType` value is invalid because it contains leading or trailing spaces.
    #[error("Dependency type contains leading or trailing spaces")]
    LeadingOrTrailingSpaces,
}
