//! Slurm sbatch command generator
//!
//! This library provides a way to generate sbatch commands for the Slurm workload manager.
//!
//! # Example
//!
//! ```
//! use sbatch_rs::{Sbatch, SbatchOption};
//! use std::str::FromStr;
//!
//! let mut sbatch_job = Sbatch::new();
//!
//! // Create a new SbatchOption using a string and append it to the Sbatch struct
//! // The append_option will return an error if the option is already present in the sbatch job.
//! let option = SbatchOption::from_str("--job-name=test").unwrap();
//! sbatch_job.append_option(option).unwrap();
//!
//! // Create SbatchOption enum variants directly and append them to the Sbatch struct
//! let option = SbatchOption::Partition("cpu".to_string());
//! sbatch_job.append_option(option).unwrap();
//!
//! // Forcibly overwrite an existing option, in this case the job name. This will not return an error.
//! let option = SbatchOption::from_str("--job-name=test2").unwrap();
//! sbatch_job.overwrite_option(option);
//!
//! // Add a script to the sbatch command. This will overwrite any existing script.
//! sbatch_job.with_script("bash_script.sh".to_string());
//!
//! // Use to_bash() to convert the Sbatch struct to a bash command. This will return an error if no script is provided.
//! // The output will be a string that represents the sbatch command.
//! assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch --job-name=test2 --partition=cpu bash_script.sh");
//!
//! ```
//!
//! # Example
//!
//! ```
//! use sbatch_rs::{Sbatch, SbatchOption, SbatchDependency, DependencyType, JobId, Variable};
//! use std::str::FromStr;
//!
//! let mut sbatch_job = Sbatch::new();
//!
//! // Add a job name to the sbatch command
//! let option = SbatchOption::JobName("test".to_string());
//! sbatch_job.append_option(option).unwrap();
//!
//! // Create a new SbatchDependency. The job id can be a number like 123, or a variable like ${job_id}
//! let mut dependency = SbatchDependency::new();
//! dependency.set_dependency(DependencyType::from_str("afterok:123").unwrap());
//! dependency.set_dependency(DependencyType::from_str("afternotok:${job_id}").unwrap());
//!
//! // You can also create dependencies from the enums directly
//! let job_id = JobId::Variable(Variable::new("jobid".to_string()).unwrap());
//! dependency.set_dependency(DependencyType::After(job_id));
//!
//! // Add the dependency to the sbatch command
//! sbatch_job.append_option(SbatchOption::Dependency(dependency)).unwrap();
//!
//! // Add a script to the sbatch command
//! sbatch_job.with_script("bash_script.sh".to_string());
//!
//! // Convert the sbatch command to a bash command
//! assert_eq!(sbatch_job.to_bash().unwrap(), "sbatch --dependency=\"after:${jobid},afternotok:${job_id},afterok:123\" --job-name=test bash_script.sh");
//!
//! ```
//!
//! # Example
//!
//! ```
//! use sbatch_rs::{Sbatch, SbatchOption};
//!
//! let mut sbatch_job = Sbatch::new();
//!
//! // Attempting to add the same option to the sbatch job will return an error
//! let option = SbatchOption::JobName("test".to_string());
//! sbatch_job.append_option(option).unwrap();
//!
//! let option = SbatchOption::JobName("test2".to_string());
//! let result = sbatch_job.append_option(option);
//! assert!(result.is_err());
//! ```
mod dependency_type;
mod sbatch;
mod sbatch_dependency;
mod sbatch_option;

pub use dependency_type::{DependencyType, DependencyTypeError};
pub use dependency_type::{JobId, JobIdError};
pub use dependency_type::{TimeDelay, TimeDelayError};
pub use dependency_type::{Variable, VariableError};
pub use sbatch::{Sbatch, SbatchError};
pub use sbatch_dependency::{SbatchDependency, SbatchDependencyError};
pub use sbatch_option::{SbatchOption, SbatchOptionError};
