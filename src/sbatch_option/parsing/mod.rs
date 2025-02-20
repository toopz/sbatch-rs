//! Parsing module for `SbatchOption`
//!
//! This module contains the parsing implementation for the `SbatchOption` enum.
//!
//! The `FromStr` trait is implemented for `SbatchOption` to allow the `SbatchOption` to be parsed from a string.
//!
//! # Example
//!
//! ```
//! use sbatch_rs::SbatchOption;
//! use std::str::FromStr;
//!
//! let option = SbatchOption::from_str("--job-name=test").unwrap();
//! assert_eq!(option, SbatchOption::JobName("test".to_string()));
//! ```

use std::num::NonZeroU32;
use std::str::FromStr;
use thiserror::Error;

use super::SbatchOption;
use crate::{SbatchDependency, SbatchDependencyError};

mod regex_match;
use regex_match::RegexMatch;

/// Errors that can occur when parsing an `SbatchOption`
///
/// This enum represents the errors that can occur when parsing an `SbatchOption`.
#[derive(Debug, Error, PartialEq)]
pub enum SbatchOptionError {
    #[error("Option cannot be an empty string")]
    EmptyString,
    #[error("Failed to parse value: {0}")]
    ParseValueError(#[from] std::num::ParseIntError),
    #[error("Failed to parse dependency: {0}")]
    ParseDependencyError(#[from] SbatchDependencyError),
    #[error("Value must be greater than 0")]
    ValueMustBeGreaterThanZero(#[from] std::num::TryFromIntError),
    #[error("Unknown argument: {0}")]
    UnknownArgument(String),
}

/// Helper function to parse a `NonZeroU32` from a string-like value
///
/// # Arguments
///
/// * `value` - Any value that can be converted into a string
///
/// # Returns
///
/// A `NonZeroU32` if the value is a valid `NonZeroU32`.
///
/// # Errors
///
/// Returns an error if the value is not a valid `NonZeroU32`.
fn parse_u32(value: impl ToString) -> Result<NonZeroU32, SbatchOptionError> {
    let parsed_value = value.to_string().parse::<u32>()?;
    NonZeroU32::try_from(parsed_value).map_err(SbatchOptionError::from)
}

impl SbatchOption {
    /// Attempts to create a new `SbatchOption` from a key-value pair
    ///
    /// This function attempts to create a new `SbatchOption` from a key-value pair. The key and value are used to create a new `SbatchOption`.
    ///
    /// # Arguments
    ///
    /// * `key` - A string that holds the key of the key-value pair
    /// * `value` - An optional string that holds the value of the key-value pair
    ///
    /// # Returns
    ///
    /// A `SbatchOption` if the key-value pair can be converted into an `SbatchOption`.
    ///
    /// # Errors
    ///
    /// Returns an error if the key-value pair cannot be converted into an `SbatchOption`.
    pub fn from_key_value(key: &str, value: Option<&str>) -> Result<Self, SbatchOptionError> {
        // let string_value = value.map(|s| s.to_string());
        let string_value = value.map(|s| s.to_string());

        match (key, string_value) {
            ("account" | "A", Some(s)) => Ok(SbatchOption::Account(s)),
            ("acctg-freq", Some(s)) => Ok(SbatchOption::AcctgFreq(s)),
            ("array" | "a", Some(s)) => Ok(SbatchOption::Array(s)),
            ("batch", Some(s)) => Ok(SbatchOption::Batch(s)),
            ("bb", Some(s)) => Ok(SbatchOption::Bb(s)),
            ("bbf", Some(s)) => Ok(SbatchOption::Bbf(s)),
            ("begin" | "b", Some(s)) => Ok(SbatchOption::Begin(s)),
            ("chdir" | "D", Some(s)) => Ok(SbatchOption::Chdir(s)),
            ("cluster-constraint", Some(s)) => Ok(SbatchOption::ClusterConstraint(s)),
            ("clusters" | "M", Some(s)) => Ok(SbatchOption::Clusters(s)),
            ("comment", Some(s)) => Ok(SbatchOption::Comment(s)),
            ("constraint" | "C", Some(s)) => Ok(SbatchOption::Constraint(s)),
            ("container", Some(s)) => Ok(SbatchOption::Container(s)),
            ("container-id", Some(s)) => Ok(SbatchOption::ContainerID(s)),
            ("contiguous", None) => Ok(SbatchOption::Contiguous),
            ("core-spec" | "S", Some(s)) => Ok(SbatchOption::CoreSpec(parse_u32(s)?)),
            ("cores-per-socket", Some(s)) => Ok(SbatchOption::CoresPerSocket(parse_u32(s)?)),
            ("cpu-freq", Some(s)) => Ok(SbatchOption::CPUFreq(s)),
            ("cpus-per-gpu", Some(s)) => Ok(SbatchOption::CPUsPerGPU(parse_u32(s)?)),
            ("cpus-per-task" | "c", Some(s)) => Ok(SbatchOption::CPUsPerTask(parse_u32(s)?)),
            ("deadline", Some(s)) => Ok(SbatchOption::Deadline(s)),
            ("delay-boot", Some(s)) => Ok(SbatchOption::DelayBoot(parse_u32(s)?)),
            ("dependency" | "d", Some(s)) => Ok(SbatchOption::Dependency(
                SbatchDependency::from_str(s.as_str())?,
            )),
            ("distribution" | "m", Some(s)) => Ok(SbatchOption::Distribution(s)),
            ("error" | "e", Some(s)) => Ok(SbatchOption::Error(s)),
            ("exclude" | "x", Some(s)) => Ok(SbatchOption::Exclude(s)),
            ("exclusive", s) => Ok(SbatchOption::Exclusive(s)),
            ("export", Some(s)) => Ok(SbatchOption::Export(s)),
            ("export-file", Some(s)) => Ok(SbatchOption::ExportFile(s)),
            ("extra", Some(s)) => Ok(SbatchOption::Extra(s)),
            ("extra-node-info" | "B", Some(s)) => Ok(SbatchOption::ExtraNodeInfo(s)),
            ("get-user-env", s) => Ok(SbatchOption::GetUserEnv(s)),
            ("gid", Some(s)) => Ok(SbatchOption::GID(s)),
            ("gpu-bind", Some(s)) => Ok(SbatchOption::GPUBind(s)),
            ("gpu-freq", Some(s)) => Ok(SbatchOption::GPUFreq(s)),
            ("gpus" | "G", Some(s)) => Ok(SbatchOption::GPUs(s)),
            ("gpus-per-node", Some(s)) => Ok(SbatchOption::GPUsPerNode(s)),
            ("gpus-per-socket", Some(s)) => Ok(SbatchOption::GPUsPerSocket(s)),
            ("gpus-per-task", Some(s)) => Ok(SbatchOption::GPUsPerTask(s)),
            ("gres", Some(s)) => Ok(SbatchOption::Gres(s)),
            ("gres-flags", Some(s)) => Ok(SbatchOption::GresFlags(s)),
            ("help" | "h", None) => Ok(SbatchOption::Help),
            ("hint", Some(s)) => Ok(SbatchOption::Hint(s)),
            ("hold" | "H", None) => Ok(SbatchOption::Hold),
            ("ignore-pbs", None) => Ok(SbatchOption::IgnorePbs),
            ("input" | "i", Some(s)) => Ok(SbatchOption::Input(s)),
            ("job-name" | "J", Some(s)) => Ok(SbatchOption::JobName(s)),
            ("kill-on-invalid-dep", Some(s)) => Ok(SbatchOption::KillOnInvalidDep(s)),
            ("licenses" | "L", Some(s)) => Ok(SbatchOption::Licenses(s)),
            ("mail-type", Some(s)) => Ok(SbatchOption::MailType(s)),
            ("mail-user", Some(s)) => Ok(SbatchOption::MailUser(s)),
            ("mcs-label", Some(s)) => Ok(SbatchOption::McsLabel(s)),
            ("mem", Some(s)) => Ok(SbatchOption::Mem(s)),
            ("mem-bind", Some(s)) => Ok(SbatchOption::MemBind(s)),
            ("mem-per-cpu", Some(s)) => Ok(SbatchOption::MemPerCPU(s)),
            ("mem-per-gpu", Some(s)) => Ok(SbatchOption::MemPerGPU(s)),
            ("min-cpus", Some(s)) => Ok(SbatchOption::MinCPUs(parse_u32(s)?)),
            ("network", Some(s)) => Ok(SbatchOption::Network(s)),
            ("nice", Some(s)) => Ok(SbatchOption::Nice(Some(s.to_string().parse::<i32>()?))),
            ("nice", None) => Ok(SbatchOption::Nice(None)),
            ("no-kill" | "k", s) => Ok(SbatchOption::NoKill(s)),
            ("no-requeue", None) => Ok(SbatchOption::NoRequeue),
            ("nodefile" | "F", Some(s)) => Ok(SbatchOption::NodeFile(s)),
            ("nodelist" | "w", Some(s)) => Ok(SbatchOption::NodeList(s)),
            ("nodes" | "N", Some(s)) => Ok(SbatchOption::Nodes(s)),
            ("ntasks" | "n", Some(s)) => Ok(SbatchOption::NTasks(parse_u32(s)?)),
            ("ntasks-per-core", Some(s)) => Ok(SbatchOption::NTasksPerCore(parse_u32(s)?)),
            ("ntasks-per-gpu", Some(s)) => Ok(SbatchOption::NTasksPerGPU(parse_u32(s)?)),
            ("ntasks-per-node", Some(s)) => Ok(SbatchOption::NTasksPerNode(parse_u32(s)?)),
            ("ntasks-per-socket", Some(s)) => Ok(SbatchOption::NTasksPerSocket(parse_u32(s)?)),
            ("oom-kill-step", s) => Ok(SbatchOption::OOMKillStep(s)),
            ("open-mode", Some(s)) => Ok(SbatchOption::OpenMode(s)),
            ("output" | "o", Some(s)) => Ok(SbatchOption::Output(s)),
            ("overcommit" | "O", None) => Ok(SbatchOption::Overcommit),
            ("oversubscribe" | "s", None) => Ok(SbatchOption::Oversubscribe),
            ("parsable", None) => Ok(SbatchOption::Parsable),
            ("partition" | "p", Some(s)) => Ok(SbatchOption::Partition(s)),
            ("prefer", Some(s)) => Ok(SbatchOption::Prefer(s)),
            ("priority", Some(s)) => Ok(SbatchOption::Priority(s)),
            ("profile", Some(s)) => Ok(SbatchOption::Profile(s)),
            ("propagate", s) => Ok(SbatchOption::Propagate(s)),
            ("qos" | "q", Some(s)) => Ok(SbatchOption::Qos(s)),
            ("quiet" | "Q", None) => Ok(SbatchOption::Quiet),
            ("reboot", None) => Ok(SbatchOption::Reboot),
            ("requeue", None) => Ok(SbatchOption::Requeue),
            ("reservation", Some(s)) => Ok(SbatchOption::Reservation(s)),
            ("resv-ports", s) => Ok(SbatchOption::ResvPorts(s)),
            ("segment", Some(s)) => Ok(SbatchOption::Segment(s)),
            ("signal", Some(s)) => Ok(SbatchOption::Signal(s)),
            ("sockets-per-node", Some(s)) => Ok(SbatchOption::SocketsPerNode(s)),
            ("spread-job", None) => Ok(SbatchOption::SpreadJob),
            ("stepmgr", None) => Ok(SbatchOption::Stepmgr),
            ("switches", Some(s)) => Ok(SbatchOption::Switches(s)),
            ("test-only", None) => Ok(SbatchOption::TestOnly),
            ("thread-spec", Some(s)) => Ok(SbatchOption::ThreadSpec(parse_u32(s)?)),
            ("threads-per-core", Some(s)) => Ok(SbatchOption::ThreadsPerCore(parse_u32(s)?)),
            ("time" | "t", Some(s)) => Ok(SbatchOption::Time(s)),
            ("time-min", Some(s)) => Ok(SbatchOption::TimeMin(s)),
            ("tmp", Some(s)) => Ok(SbatchOption::Tmp(s)),
            ("tres-bind", Some(s)) => Ok(SbatchOption::TresBind(s)),
            ("tres-per-task", Some(s)) => Ok(SbatchOption::TresPerTask(s)),
            ("uid", Some(s)) => Ok(SbatchOption::UID(s)),
            ("usage", None) => Ok(SbatchOption::Usage),
            ("use-min-nodes", None) => Ok(SbatchOption::UseMinNodes),
            ("verbose" | "v", None) => Ok(SbatchOption::Verbose),
            ("version" | "V", None) => Ok(SbatchOption::Version),
            ("wait", None) => Ok(SbatchOption::Wait),
            ("wait-all-nodes", Some(s)) => Ok(SbatchOption::WaitAllNodes(s)),
            ("wckey", Some(s)) => Ok(SbatchOption::WCKey(s)),
            ("wrap", Some(s)) => Ok(SbatchOption::Wrap(s)),
            _ => Err(SbatchOptionError::UnknownArgument(format!(
                "{:?} {:?}",
                key, value
            ))),
        }
    }

    /// Checks if the option is the same variant as another option
    ///
    /// This function checks if the option is the same variant as another option. This is useful for checking if an option is already in a list.
    ///
    /// # Arguments
    ///
    /// * `other` - Another `SbatchOption` to compare against
    ///
    /// # Returns
    ///
    /// A boolean indicating if the option is the same variant as the other option.
    pub fn is_same_variant(&self, other: &SbatchOption) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl FromStr for SbatchOption {
    type Err = SbatchOptionError;

    /// Parses a `SbatchOption` from a string
    ///
    /// This function parses a `SbatchOption` from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - A string that holds the key-value pair
    ///
    /// # Returns
    ///
    /// A `SbatchOption` if the input string can be parsed and converted into an `SbatchOption`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input string cannot be parsed into a `SbatchOption`.
    fn from_str(s: &str) -> Result<Self, SbatchOptionError> {
        let regex_match = RegexMatch::from_str(s)
            .ok_or_else(|| SbatchOptionError::UnknownArgument(s.to_string()))?;

        SbatchOption::from_key_value(regex_match.key(), regex_match.value())
    }
}
