//! Display implementation for `SbatchOption`
//!
//! This module contains the display implementation for the `SbatchOption` enum.
//!
//! The `Display` trait is implemented for `SbatchOption` to allow the `SbatchOption` to be converted into a string for display purposes.
//!
//! # Example
//!
//! ```
//! use sbatch_rs::SbatchOption;
//!
//! let option = SbatchOption::JobName("test".to_string());
//! assert_eq!(option.to_string(), "--job-name=test");
//! ```

use super::SbatchOption;
use std::fmt::Display;

/// Display implementation for `SbatchOption`
impl Display for SbatchOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SbatchOption::Account(value) => write!(f, r#"--account={}"#, value),
            SbatchOption::AcctgFreq(value) => write!(f, "--acctg-freq={}", value),
            SbatchOption::Array(value) => write!(f, "--array={}", value),
            SbatchOption::Batch(value) => write!(f, "--batch={}", value),
            SbatchOption::Bb(value) => write!(f, "--bb={}", value),
            SbatchOption::Bbf(value) => write!(f, "--bbf={}", value),
            SbatchOption::Begin(value) => write!(f, "--begin={}", value),
            SbatchOption::Chdir(value) => write!(f, "--chdir={}", value),
            SbatchOption::ClusterConstraint(value) => write!(f, "--cluster-constraint={}", value),
            SbatchOption::Clusters(value) => write!(f, "--clusters={}", value),
            SbatchOption::Comment(value) => write!(f, "--comment={}", value),
            SbatchOption::Constraint(value) => write!(f, "--constraint={}", value),
            SbatchOption::Container(value) => write!(f, "--container={}", value),
            SbatchOption::ContainerID(value) => write!(f, "--container-id={}", value),
            SbatchOption::Contiguous => write!(f, "--contiguous"),
            SbatchOption::CoreSpec(value) => write!(f, "--core-spec={}", value),
            SbatchOption::CoresPerSocket(value) => write!(f, "--cores-per-socket={}", value),
            SbatchOption::CPUFreq(value) => write!(f, "--cpu-freq={}", value),
            SbatchOption::CPUsPerGPU(value) => write!(f, "--cpus-per-gpu={}", value),
            SbatchOption::CPUsPerTask(value) => write!(f, "--cpus-per-task={}", value),
            SbatchOption::Deadline(value) => write!(f, "--deadline={}", value),
            SbatchOption::DelayBoot(value) => write!(f, "--delay-boot={}", value),
            SbatchOption::Dependency(value) => write!(f, "--dependency={}", value),
            SbatchOption::Distribution(value) => write!(f, "--distribution={}", value),
            SbatchOption::Error(value) => write!(f, "--error={}", value),
            SbatchOption::Exclude(value) => write!(f, "--exclude={}", value),
            SbatchOption::Exclusive(Some(value)) => write!(f, "--exclusive={}", value),
            SbatchOption::Exclusive(None) => write!(f, "--exclusive"),
            SbatchOption::Export(value) => write!(f, "--export={}", value),
            SbatchOption::ExportFile(value) => write!(f, "--export-file={}", value),
            SbatchOption::Extra(value) => write!(f, "--extra={}", value),
            SbatchOption::ExtraNodeInfo(value) => write!(f, "--extra-node-info={}", value),
            SbatchOption::GetUserEnv(Some(value)) => write!(f, "--get-user-env={}", value),
            SbatchOption::GetUserEnv(None) => write!(f, "--get-user-env"),
            SbatchOption::GID(value) => write!(f, "--gid={}", value),
            SbatchOption::GPUBind(value) => write!(f, "--gpu-bind={}", value),
            SbatchOption::GPUFreq(value) => write!(f, "--gpu-freq={}", value),
            SbatchOption::GPUs(value) => write!(f, "--gpus={}", value),
            SbatchOption::GPUsPerNode(value) => write!(f, "--gpus-per-node={}", value),
            SbatchOption::GPUsPerSocket(value) => write!(f, "--gpus-per-socket={}", value),
            SbatchOption::GPUsPerTask(value) => write!(f, "--gpus-per-task={}", value),
            SbatchOption::Gres(value) => write!(f, "--gres={}", value),
            SbatchOption::GresFlags(value) => write!(f, "--gres-flags={}", value),
            SbatchOption::Help => write!(f, "--help"),
            SbatchOption::Hint(value) => write!(f, "--hint={}", value),
            SbatchOption::Hold => write!(f, "--hold"),
            SbatchOption::IgnorePbs => write!(f, "--ignore-pbs"),
            SbatchOption::Input(value) => write!(f, "--input={}", value),
            SbatchOption::JobName(value) => write!(f, "--job-name={}", value),
            SbatchOption::KillOnInvalidDep(value) => write!(f, "--kill-on-invalid-dep={}", value),
            SbatchOption::Licenses(value) => write!(f, "--licenses={}", value),
            SbatchOption::MailType(value) => write!(f, "--mail-type={}", value),
            SbatchOption::MailUser(value) => write!(f, "--mail-user={}", value),
            SbatchOption::McsLabel(value) => write!(f, "--mcs-label={}", value),
            SbatchOption::Mem(value) => write!(f, "--mem={}", value),
            SbatchOption::MemBind(value) => write!(f, "--mem-bind={}", value),
            SbatchOption::MemPerCPU(value) => write!(f, "--mem-per-cpu={}", value),
            SbatchOption::MemPerGPU(value) => write!(f, "--mem-per-gpu={}", value),
            SbatchOption::MinCPUs(value) => write!(f, "--min-cpus={}", value),
            SbatchOption::Network(value) => write!(f, "--network={}", value),
            SbatchOption::Nice(Some(value)) => write!(f, "--nice={}", value),
            SbatchOption::Nice(None) => write!(f, "--nice"),
            SbatchOption::NoKill(Some(value)) => write!(f, "--no-kill={}", value),
            SbatchOption::NoKill(None) => write!(f, "--no-kill"),
            SbatchOption::NoRequeue => write!(f, "--no-requeue"),
            SbatchOption::NodeFile(value) => write!(f, "--nodefile={}", value),
            SbatchOption::NodeList(value) => write!(f, "--nodelist={}", value),
            SbatchOption::Nodes(value) => write!(f, "--nodes={}", value),
            SbatchOption::NTasks(value) => write!(f, "--ntasks={}", value),
            SbatchOption::NTasksPerCore(value) => write!(f, "--ntasks-per-core={}", value),
            SbatchOption::NTasksPerGPU(value) => write!(f, "--ntasks-per-gpu={}", value),
            SbatchOption::NTasksPerNode(value) => write!(f, "--ntasks-per-node={}", value),
            SbatchOption::NTasksPerSocket(value) => write!(f, "--ntasks-per-socket={}", value),
            SbatchOption::OOMKillStep(Some(value)) => write!(f, "--oom-kill-step={}", value),
            SbatchOption::OOMKillStep(None) => write!(f, "--oom-kill-step"),
            SbatchOption::OpenMode(value) => write!(f, "--open-mode={}", value),
            SbatchOption::Output(value) => write!(f, "--output={}", value),
            SbatchOption::Overcommit => write!(f, "--overcommit"),
            SbatchOption::Oversubscribe => write!(f, "--oversubscribe"),
            SbatchOption::Parsable => write!(f, "--parsable"),
            SbatchOption::Partition(value) => write!(f, "--partition={}", value),
            SbatchOption::Prefer(value) => write!(f, "--prefer={}", value),
            SbatchOption::Priority(value) => write!(f, "--priority={}", value),
            SbatchOption::Profile(value) => write!(f, "--profile={}", value),
            SbatchOption::Propagate(Some(value)) => write!(f, "--propagate={}", value),
            SbatchOption::Propagate(None) => write!(f, "--propagate"),
            SbatchOption::Qos(value) => write!(f, "--qos={}", value),
            SbatchOption::Quiet => write!(f, "--quiet"),
            SbatchOption::Reboot => write!(f, "--reboot"),
            SbatchOption::Requeue => write!(f, "--requeue"),
            SbatchOption::Reservation(value) => write!(f, "--reservation={}", value),
            SbatchOption::ResvPorts(Some(value)) => write!(f, "--resv-ports={}", value),
            SbatchOption::ResvPorts(None) => write!(f, "--resv-ports"),
            SbatchOption::Segment(value) => write!(f, "--segment={}", value),
            SbatchOption::Signal(value) => write!(f, "--signal={}", value),
            SbatchOption::SocketsPerNode(value) => write!(f, "--sockets-per-node={}", value),
            SbatchOption::SpreadJob => write!(f, "--spread-job"),
            SbatchOption::Stepmgr => write!(f, "--stepmgr"),
            SbatchOption::Switches(value) => write!(f, "--switches={}", value),
            SbatchOption::TestOnly => write!(f, "--test-only"),
            SbatchOption::ThreadSpec(value) => write!(f, "--thread-spec={}", value),
            SbatchOption::ThreadsPerCore(value) => write!(f, "--threads-per-core={}", value),
            SbatchOption::Time(value) => write!(f, "--time={}", value),
            SbatchOption::TimeMin(value) => write!(f, "--time-min={}", value),
            SbatchOption::Tmp(value) => write!(f, "--tmp={}", value),
            SbatchOption::TresBind(value) => write!(f, "--tres-bind={}", value),
            SbatchOption::TresPerTask(value) => write!(f, "--tres-per-task={}", value),
            SbatchOption::UID(value) => write!(f, "--uid={}", value),
            SbatchOption::Usage => write!(f, "--usage"),
            SbatchOption::UseMinNodes => write!(f, "--use-min-nodes"),
            SbatchOption::Verbose => write!(f, "--verbose"),
            SbatchOption::Version => write!(f, "--version"),
            SbatchOption::Wait => write!(f, "--wait"),
            SbatchOption::WaitAllNodes(value) => write!(f, "--wait-all-nodes={}", value),
            SbatchOption::WCKey(value) => write!(f, "--wckey={}", value),
            SbatchOption::Wrap(value) => write!(f, "--wrap={}", value),
        }
    }
}
