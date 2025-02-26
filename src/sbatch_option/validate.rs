use super::{SbatchOption, SbatchOptionError};

// Helper function to validate a string.
// This function checks if the string is empty or contains leading or trailing spaces.
fn validate_str(s: &str) -> Result<(), SbatchOptionError> {
    if s.trim().is_empty() {
        Err(SbatchOptionError::EmptyString)
    } else if s != s.trim() {
        Err(SbatchOptionError::LeadingOrTrailingSpaces)
    } else {
        Ok(())
    }
}

impl SbatchOption {
    /// Validates the sbatch option.
    ///
    /// # Returns
    ///
    /// This function returns `Ok(())` if the sbatch option is valid, otherwise it returns a `SbatchOptionError`.
    ///
    /// # Errors
    ///
    /// This function returns a `SbatchOptionError` if the sbatch option is invalid.
    /// The following are considered invalid:
    /// - An empty string
    /// - A string that contains leading or trailing spaces
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::SbatchOption;
    ///
    /// // Valid: mapped to `--account=account`
    /// let sbatch_option = SbatchOption::Account("account".to_string());
    /// assert!(sbatch_option.validate().is_ok());
    ///
    /// // Invalid: empty string
    /// let sbatch_option = SbatchOption::Account("".to_string());
    /// assert!(sbatch_option.validate().is_err());
    ///
    /// // Invalid: leading or trailing spaces
    /// let sbatch_option = SbatchOption::Account(" account ".to_string());
    /// assert!(sbatch_option.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), SbatchOptionError> {
        match self {
            SbatchOption::Account(value) => validate_str(value),
            SbatchOption::AcctgFreq(value) => validate_str(value),
            SbatchOption::Array(value) => validate_str(value),
            SbatchOption::Batch(value) => validate_str(value),
            SbatchOption::Bb(value) => validate_str(value),
            SbatchOption::Bbf(value) => validate_str(value),
            SbatchOption::Begin(value) => validate_str(value),
            SbatchOption::Chdir(value) => validate_str(value),
            SbatchOption::ClusterConstraint(value) => validate_str(value),
            SbatchOption::Clusters(value) => validate_str(value),
            SbatchOption::Comment(value) => validate_str(value),
            SbatchOption::Constraint(value) => validate_str(value),
            SbatchOption::Container(value) => validate_str(value),
            SbatchOption::ContainerID(value) => validate_str(value),
            SbatchOption::Contiguous => Ok(()),
            SbatchOption::CoreSpec(value) => validate_str(value),
            SbatchOption::CoresPerSocket(value) => validate_str(value),
            SbatchOption::CPUFreq(value) => validate_str(value),
            SbatchOption::CPUsPerGPU(value) => validate_str(value),
            SbatchOption::CPUsPerTask(value) => validate_str(value),
            SbatchOption::Deadline(value) => validate_str(value),
            SbatchOption::DelayBoot(value) => validate_str(value),
            SbatchOption::Dependency(value) => validate_str(value),
            SbatchOption::Distribution(value) => validate_str(value),
            SbatchOption::Error(value) => validate_str(value),
            SbatchOption::Exclude(value) => validate_str(value),
            SbatchOption::Exclusive(Some(value)) => validate_str(value),
            SbatchOption::Exclusive(None) => Ok(()),
            SbatchOption::Export(value) => validate_str(value),
            SbatchOption::ExportFile(value) => validate_str(value),
            SbatchOption::Extra(value) => validate_str(value),
            SbatchOption::ExtraNodeInfo(value) => validate_str(value),
            SbatchOption::GetUserEnv(Some(value)) => validate_str(value),
            SbatchOption::GetUserEnv(None) => Ok(()),
            SbatchOption::GID(value) => validate_str(value),
            SbatchOption::GPUBind(value) => validate_str(value),
            SbatchOption::GPUFreq(value) => validate_str(value),
            SbatchOption::GPUs(value) => validate_str(value),
            SbatchOption::GPUsPerNode(value) => validate_str(value),
            SbatchOption::GPUsPerSocket(value) => validate_str(value),
            SbatchOption::GPUsPerTask(value) => validate_str(value),
            SbatchOption::Gres(value) => validate_str(value),
            SbatchOption::GresFlags(value) => validate_str(value),
            SbatchOption::Help => Ok(()),
            SbatchOption::Hint(value) => validate_str(value),
            SbatchOption::Hold => Ok(()),
            SbatchOption::IgnorePbs => Ok(()),
            SbatchOption::Input(value) => validate_str(value),
            SbatchOption::JobName(value) => validate_str(value),
            SbatchOption::KillOnInvalidDep(value) => validate_str(value),
            SbatchOption::Licenses(value) => validate_str(value),
            SbatchOption::MailType(value) => validate_str(value),
            SbatchOption::MailUser(value) => validate_str(value),
            SbatchOption::McsLabel(value) => validate_str(value),
            SbatchOption::Mem(value) => validate_str(value),
            SbatchOption::MemBind(value) => validate_str(value),
            SbatchOption::MemPerCPU(value) => validate_str(value),
            SbatchOption::MemPerGPU(value) => validate_str(value),
            SbatchOption::MinCPUs(value) => validate_str(value),
            SbatchOption::Network(value) => validate_str(value),
            SbatchOption::Nice(Some(value)) => validate_str(value),
            SbatchOption::Nice(None) => Ok(()),
            SbatchOption::NoKill(Some(value)) => validate_str(value),
            SbatchOption::NoKill(None) => Ok(()),
            SbatchOption::NoRequeue => Ok(()),
            SbatchOption::NodeFile(value) => validate_str(value),
            SbatchOption::NodeList(value) => validate_str(value),
            SbatchOption::Nodes(value) => validate_str(value),
            SbatchOption::NTasks(value) => validate_str(value),
            SbatchOption::NTasksPerCore(value) => validate_str(value),
            SbatchOption::NTasksPerGPU(value) => validate_str(value),
            SbatchOption::NTasksPerNode(value) => validate_str(value),
            SbatchOption::NTasksPerSocket(value) => validate_str(value),
            SbatchOption::OOMKillStep(Some(value)) => validate_str(value),
            SbatchOption::OOMKillStep(None) => Ok(()),
            SbatchOption::OpenMode(value) => validate_str(value),
            SbatchOption::Output(value) => validate_str(value),
            SbatchOption::Overcommit => Ok(()),
            SbatchOption::Oversubscribe => Ok(()),
            SbatchOption::Parsable => Ok(()),
            SbatchOption::Partition(value) => validate_str(value),
            SbatchOption::Prefer(value) => validate_str(value),
            SbatchOption::Priority(value) => validate_str(value),
            SbatchOption::Profile(value) => validate_str(value),
            SbatchOption::Propagate(Some(value)) => validate_str(value),
            SbatchOption::Propagate(None) => Ok(()),
            SbatchOption::Qos(value) => validate_str(value),
            SbatchOption::Quiet => Ok(()),
            SbatchOption::Reboot => Ok(()),
            SbatchOption::Requeue => Ok(()),
            SbatchOption::Reservation(value) => validate_str(value),
            SbatchOption::ResvPorts(Some(value)) => validate_str(value),
            SbatchOption::ResvPorts(None) => Ok(()),
            SbatchOption::Segment(value) => validate_str(value),
            SbatchOption::Signal(value) => validate_str(value),
            SbatchOption::SocketsPerNode(value) => validate_str(value),
            SbatchOption::SpreadJob => Ok(()),
            SbatchOption::Stepmgr => Ok(()),
            SbatchOption::Switches(value) => validate_str(value),
            SbatchOption::TestOnly => Ok(()),
            SbatchOption::ThreadSpec(value) => validate_str(value),
            SbatchOption::ThreadsPerCore(value) => validate_str(value),
            SbatchOption::Time(value) => validate_str(value),
            SbatchOption::TimeMin(value) => validate_str(value),
            SbatchOption::Tmp(value) => validate_str(value),
            SbatchOption::TresBind(value) => validate_str(value),
            SbatchOption::TresPerTask(value) => validate_str(value),
            SbatchOption::UID(value) => validate_str(value),
            SbatchOption::Usage => Ok(()),
            SbatchOption::UseMinNodes => Ok(()),
            SbatchOption::Verbose => Ok(()),
            SbatchOption::Version => Ok(()),
            SbatchOption::Wait => Ok(()),
            SbatchOption::WaitAllNodes(value) => validate_str(value),
            SbatchOption::WCKey(value) => validate_str(value),
            SbatchOption::Wrap(_) => Ok(()), // Allow --wrap="" to be empty
        }
    }
}
