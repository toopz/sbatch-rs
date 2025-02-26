use super::DependencyType;

impl std::fmt::Display for DependencyType {
    /// Display the `DependencyType` value as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbatch_rs::DependencyType;
    ///
    /// // Display the `After` variant
    /// let dependency_type = DependencyType::After("123".to_string());
    /// assert_eq!(dependency_type.to_string(), "after:123");
    ///
    /// // Display the `AfterTimeDelay` variant
    /// let dependency_type = DependencyType::AfterTimeDelay("123".to_string(), "10".to_string());
    /// assert_eq!(dependency_type.to_string(), "after:123+10");
    ///
    /// // Display the `AfterAny` variant
    /// let dependency_type = DependencyType::AfterAny("123".to_string());
    /// assert_eq!(dependency_type.to_string(), "afterany:123");
    ///
    /// // Display the `Singleton` variant
    /// let dependency_type = DependencyType::Singleton;
    /// assert_eq!(dependency_type.to_string(), "singleton");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyType::After(job_id) => write!(f, "after:{}", job_id.trim()),
            DependencyType::AfterTimeDelay(job_id, time_delay) => {
                write!(f, "after:{}+{}", job_id.trim(), time_delay.trim())
            }
            DependencyType::AfterAny(job_id) => write!(f, "afterany:{}", job_id.trim()),
            DependencyType::AfterBurstBuffer(job_id) => {
                write!(f, "afterburstbuffer:{}", job_id.trim())
            }
            DependencyType::AfterCorr(job_id) => write!(f, "aftercorr:{}", job_id.trim()),
            DependencyType::AfterNotOk(job_id) => write!(f, "afternotok:{}", job_id.trim()),
            DependencyType::AfterOk(job_id) => write!(f, "afterok:{}", job_id.trim()),
            DependencyType::Singleton => write!(f, "singleton"),
        }
    }
}
