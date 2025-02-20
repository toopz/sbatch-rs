//! Sbatch options enum and parser
//!
//! This module provides an enum for representing sbatch options and a parser for converting
//! For a full description of the sbatch options, see the slurm documentation: <https://slurm.schedmd.com/sbatch.html>
//!
//! Where possible, the enum variants are named after the long form of the sbatch option. For example, the `--job-name` option is represented by the `JobName` variant.
//! The values of the enum variants are stored as `String`, or `NonZeroU32` where appropriate to ensure that the values are valid.
//! Some options have additional constraints, such as the `--dependency` option which requires a list of dependencies. These options are represented by a separate struct.
mod display_impl;
mod parsing;

use std::num::NonZeroU32;

use crate::SbatchDependency;

pub use parsing::SbatchOptionError;

/// Represents an sbatch option
///
/// The `SbatchOption` enum represents an sbatch option. The enum variants are named after the long form of the sbatch option.
/// The values of the enum variants are stored as `String`, or `NonZeroU32` where appropriate to ensure that the values are valid.
/// Some options have additional constraints, such as the `--dependency` option which requires a list of dependencies. These options are represented by a separate struct.
/// For a full description of the sbatch options, see the slurm documentation: <https://slurm.schedmd.com/sbatch.html>
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SbatchOption {
    Account(String),
    AcctgFreq(String),
    Array(String),
    Batch(String),
    Bb(String),
    Bbf(String),
    Begin(String),
    Chdir(String),
    ClusterConstraint(String),
    Clusters(String),
    Comment(String),
    Constraint(String),
    Container(String),
    ContainerID(String),
    Contiguous,
    CoreSpec(NonZeroU32),
    CoresPerSocket(NonZeroU32),
    CPUFreq(String),
    CPUsPerGPU(NonZeroU32),
    CPUsPerTask(NonZeroU32),
    Deadline(String),
    DelayBoot(NonZeroU32),
    Dependency(SbatchDependency),
    Distribution(String),
    Error(String),
    Exclude(String),
    Exclusive(Option<String>),
    Export(String),
    ExportFile(String),
    Extra(String),
    ExtraNodeInfo(String),
    GetUserEnv(Option<String>),
    GID(String),
    GPUBind(String),
    GPUFreq(String),
    GPUs(String),
    GPUsPerNode(String),
    GPUsPerSocket(String),
    GPUsPerTask(String),
    Gres(String),
    GresFlags(String),
    Help,
    Hint(String),
    Hold,
    IgnorePbs,
    Input(String),
    JobName(String),
    KillOnInvalidDep(String),
    Licenses(String),
    MailType(String),
    MailUser(String),
    McsLabel(String),
    Mem(String),
    MemBind(String),
    MemPerCPU(String),
    MemPerGPU(String),
    MinCPUs(NonZeroU32),
    Network(String),
    Nice(Option<i32>),
    NoKill(Option<String>),
    NoRequeue,
    NodeFile(String),
    NodeList(String),
    Nodes(String),
    NTasks(NonZeroU32),
    NTasksPerCore(NonZeroU32),
    NTasksPerGPU(NonZeroU32),
    NTasksPerNode(NonZeroU32),
    NTasksPerSocket(NonZeroU32),
    OOMKillStep(Option<String>),
    OpenMode(String),
    Output(String),
    Overcommit,
    Oversubscribe,
    Parsable,
    Partition(String),
    Prefer(String),
    Priority(String),
    Profile(String),
    Propagate(Option<String>),
    Qos(String),
    Quiet,
    Reboot,
    Requeue,
    Reservation(String),
    ResvPorts(Option<String>),
    Segment(String),
    Signal(String),
    SocketsPerNode(String),
    SpreadJob,
    Stepmgr,
    Switches(String),
    TestOnly,
    ThreadSpec(NonZeroU32),
    ThreadsPerCore(NonZeroU32),
    Time(String),
    TimeMin(String),
    Tmp(String),
    TresBind(String),
    TresPerTask(String),
    UID(String),
    Usage,
    UseMinNodes,
    Verbose,
    Version,
    Wait,
    WaitAllNodes(String),
    WCKey(String),
    Wrap(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case(r#"--account="account""#, SbatchOption::Account("account".to_string()), r#"--account="account""#)]
    #[case(r#"--account "account""#, SbatchOption::Account("account".to_string()), r#"--account="account""#)]
    #[case(r#"-A "account""#, SbatchOption::Account("account".to_string()), r#"--account="account""#)]
    #[case(r#"--acctg-freq="acctg_freq""#, SbatchOption::AcctgFreq("acctg_freq".to_string()), r#"--acctg-freq="acctg_freq""#)]
    #[case(r#"--array="array""#, SbatchOption::Array("array".to_string()), r#"--array="array""#)]
    #[case(r#"-a "array""#, SbatchOption::Array("array".to_string()), r#"--array="array""#)]
    #[case(r#"--batch="batch""#, SbatchOption::Batch("batch".to_string()), r#"--batch="batch""#)]
    #[case(r#"--bb="bb""#, SbatchOption::Bb("bb".to_string()), r#"--bb="bb""#)]
    #[case(r#"--bbf="bbf""#, SbatchOption::Bbf("bbf".to_string()), r#"--bbf="bbf""#)]
    #[case(r#"--begin="begin""#, SbatchOption::Begin("begin".to_string()), r#"--begin="begin""#)]
    #[case(r#"-b "begin""#, SbatchOption::Begin("begin".to_string()), r#"--begin="begin""#)]
    #[case(r#"--chdir="chdir""#, SbatchOption::Chdir("chdir".to_string()), r#"--chdir="chdir""#)]
    #[case(r#"-D "chdir""#, SbatchOption::Chdir("chdir".to_string()), r#"--chdir="chdir""#)]
    #[case(r#"--cluster-constraint="cluster_constraint""#, SbatchOption::ClusterConstraint("cluster_constraint".to_string()), r#"--cluster-constraint="cluster_constraint""#)]
    #[case(r#"--clusters="clusters""#, SbatchOption::Clusters("clusters".to_string()), r#"--clusters="clusters""#)]
    #[case(r#"-M "clusters""#, SbatchOption::Clusters("clusters".to_string()), r#"--clusters="clusters""#)]
    #[case(r#"--comment="comment""#, SbatchOption::Comment("comment".to_string()), r#"--comment="comment""#)]
    #[case(r#"--constraint="constraint""#, SbatchOption::Constraint("constraint".to_string()), r#"--constraint="constraint""#)]
    #[case(r#"-C "constraint""#, SbatchOption::Constraint("constraint".to_string()), r#"--constraint="constraint""#)]
    #[case(r#"--container="container""#, SbatchOption::Container("container".to_string()), r#"--container="container""#)]
    #[case(r#"--container-id="container_id""#, SbatchOption::ContainerID("container_id".to_string()), r#"--container-id="container_id""#)]
    #[case(r#"--contiguous"#, SbatchOption::Contiguous, r#"--contiguous"#)]
    #[case(r#"--core-spec=1"#, SbatchOption::CoreSpec(NonZeroU32::new(1).unwrap()), r#"--core-spec=1"#)]
    #[case(r#"--cores-per-socket=1"#, SbatchOption::CoresPerSocket(NonZeroU32::new(1).unwrap()), r#"--cores-per-socket=1"#)]
    #[case(r#"--cpu-freq="cpu_freq""#, SbatchOption::CPUFreq("cpu_freq".to_string()), r#"--cpu-freq="cpu_freq""#)]
    #[case(r#"--cpus-per-gpu=1"#, SbatchOption::CPUsPerGPU(NonZeroU32::new(1).unwrap()), r#"--cpus-per-gpu=1"#)]
    #[case(r#"--cpus-per-task=1"#, SbatchOption::CPUsPerTask(NonZeroU32::new(1).unwrap()), r#"--cpus-per-task=1"#)]
    #[case(r#"--deadline="deadline""#, SbatchOption::Deadline("deadline".to_string()), r#"--deadline="deadline""#)]
    #[case(r#"--delay-boot=1"#, SbatchOption::DelayBoot(NonZeroU32::new(1).unwrap()), r#"--delay-boot=1"#)]
    #[case(r#"--dependency="afterok:1:2:3""#, SbatchOption::Dependency(SbatchDependency::from_str("afterok:1:2:3").unwrap()), r#"--dependency="afterok:1,afterok:2,afterok:3""#)]
    #[case(r#"-d "afterok:1:2:3""#, SbatchOption::Dependency(SbatchDependency::from_str("afterok:1:2:3").unwrap()), r#"--dependency="afterok:1,afterok:2,afterok:3""#)]
    #[case(r#"--distribution="distribution""#, SbatchOption::Distribution("distribution".to_string()), r#"--distribution="distribution""#)]
    #[case(r#"-m "distribution""#, SbatchOption::Distribution("distribution".to_string()), r#"--distribution="distribution""#)]
    #[case(r#"--error="error""#, SbatchOption::Error("error".to_string()), r#"--error="error""#)]
    #[case(r#"-e "error""#, SbatchOption::Error("error".to_string()), r#"--error="error""#)]
    #[case(r#"--exclude="exclude""#, SbatchOption::Exclude("exclude".to_string()), r#"--exclude="exclude""#)]
    #[case(r#"-x "exclude""#, SbatchOption::Exclude("exclude".to_string()), r#"--exclude="exclude""#)]
    #[case(r#"--exclusive="exclusive""#, SbatchOption::Exclusive(Some("exclusive".to_string())), r#"--exclusive="exclusive""#)]
    #[case(r#"--exclusive"#, SbatchOption::Exclusive(None), r#"--exclusive"#)]
    #[case(r#"--export="export""#, SbatchOption::Export("export".to_string()), r#"--export="export""#)]
    #[case(r#"--export-file="export_file""#, SbatchOption::ExportFile("export_file".to_string()), r#"--export-file="export_file""#)]
    #[case(r#"--extra="extra""#, SbatchOption::Extra("extra".to_string()), r#"--extra="extra""#)]
    #[case(r#"--extra-node-info="extra_node_info""#, SbatchOption::ExtraNodeInfo("extra_node_info".to_string()), r#"--extra-node-info="extra_node_info""#)]
    #[case(r#"--get-user-env="get_user_env""#, SbatchOption::GetUserEnv(Some("get_user_env".to_string())), r#"--get-user-env="get_user_env""#)]
    #[case(
        r#"--get-user-env"#,
        SbatchOption::GetUserEnv(None),
        r#"--get-user-env"#
    )]
    #[case(r#"--gid="gid""#, SbatchOption::GID("gid".to_string()), r#"--gid="gid""#)]
    #[case(r#"--gpu-bind="gpu_bind""#, SbatchOption::GPUBind("gpu_bind".to_string()), r#"--gpu-bind="gpu_bind""#)]
    #[case(r#"--gpu-freq="gpu_freq""#, SbatchOption::GPUFreq("gpu_freq".to_string()), r#"--gpu-freq="gpu_freq""#)]
    #[case(r#"--gpus="gpus""#, SbatchOption::GPUs("gpus".to_string()), r#"--gpus="gpus""#)]
    #[case(r#"-G "gpus""#, SbatchOption::GPUs("gpus".to_string()), r#"--gpus="gpus""#)]
    #[case(r#"--gpus-per-node="gpus_per_node""#, SbatchOption::GPUsPerNode("gpus_per_node".to_string()), r#"--gpus-per-node="gpus_per_node""#)]
    #[case(r#"--gpus-per-socket="gpus_per_socket""#, SbatchOption::GPUsPerSocket("gpus_per_socket".to_string()), r#"--gpus-per-socket="gpus_per_socket""#)]
    #[case(r#"--gpus-per-task="gpus_per_task""#, SbatchOption::GPUsPerTask("gpus_per_task".to_string()), r#"--gpus-per-task="gpus_per_task""#)]
    #[case(r#"--gres="gres""#, SbatchOption::Gres("gres".to_string()), r#"--gres="gres""#)]
    #[case(r#"--gres-flags="gres_flags""#, SbatchOption::GresFlags("gres_flags".to_string()), r#"--gres-flags="gres_flags""#)]
    #[case(r#"--help"#, SbatchOption::Help, r#"--help"#)]
    #[case(r#"--hint="hint""#, SbatchOption::Hint("hint".to_string()), r#"--hint="hint""#)]
    #[case(r#"--hold"#, SbatchOption::Hold, r#"--hold"#)]
    #[case(r#"--ignore-pbs"#, SbatchOption::IgnorePbs, r#"--ignore-pbs"#)]
    #[case(r#"--input="input""#, SbatchOption::Input("input".to_string()), r#"--input="input""#)]
    #[case(r#"-i "input""#, SbatchOption::Input("input".to_string()), r#"--input="input""#)]
    #[case(r#"--job-name="job_name""#, SbatchOption::JobName("job_name".to_string()), r#"--job-name="job_name""#)]
    #[case(r#"-J "job_name""#, SbatchOption::JobName("job_name".to_string()), r#"--job-name="job_name""#)]
    #[case(r#"--kill-on-invalid-dep="kill_on_invalid_dep""#, SbatchOption::KillOnInvalidDep("kill_on_invalid_dep".to_string()), r#"--kill-on-invalid-dep="kill_on_invalid_dep""#)]
    #[case(r#"--licenses="licenses""#, SbatchOption::Licenses("licenses".to_string()), r#"--licenses="licenses""#)]
    #[case(r#"-L "licenses""#, SbatchOption::Licenses("licenses".to_string()), r#"--licenses="licenses""#)]
    #[case(r#"--mail-type="mail_type""#, SbatchOption::MailType("mail_type".to_string()), r#"--mail-type="mail_type""#)]
    #[case(r#"--mail-user="mail_user""#, SbatchOption::MailUser("mail_user".to_string()), r#"--mail-user="mail_user""#)]
    #[case(r#"--mcs-label="mcs_label""#, SbatchOption::McsLabel("mcs_label".to_string()), r#"--mcs-label="mcs_label""#)]
    #[case(r#"--mem="mem""#, SbatchOption::Mem("mem".to_string()), r#"--mem="mem""#)]
    #[case(r#"--mem-bind="mem_bind""#, SbatchOption::MemBind("mem_bind".to_string()), r#"--mem-bind="mem_bind""#)]
    #[case(r#"--mem-per-cpu="mem_per_cpu""#, SbatchOption::MemPerCPU("mem_per_cpu".to_string()), r#"--mem-per-cpu="mem_per_cpu""#)]
    #[case(r#"--mem-per-gpu="mem_per_gpu""#, SbatchOption::MemPerGPU("mem_per_gpu".to_string()), r#"--mem-per-gpu="mem_per_gpu""#)]
    #[case(r#"--min-cpus=1"#, SbatchOption::MinCPUs(NonZeroU32::new(1).unwrap()), r#"--min-cpus=1"#)]
    #[case(r#"--network="network""#, SbatchOption::Network("network".to_string()), r#"--network="network""#)]
    #[case(r#"--nice=1"#, SbatchOption::Nice(Some(1)), r#"--nice=1"#)]
    #[case(r#"--nice"#, SbatchOption::Nice(None), r#"--nice"#)]
    #[case(r#"--no-kill="no_kill""#, SbatchOption::NoKill(Some("no_kill".to_string())), r#"--no-kill="no_kill""#)]
    #[case(r#"--no-kill"#, SbatchOption::NoKill(None), r#"--no-kill"#)]
    #[case(r#"--no-requeue"#, SbatchOption::NoRequeue, r#"--no-requeue"#)]
    #[case(r#"--nodefile="node_file""#, SbatchOption::NodeFile("node_file".to_string()), r#"--nodefile="node_file""#)]
    #[case(r#"--nodelist="node_list""#, SbatchOption::NodeList("node_list".to_string()), r#"--nodelist="node_list""#)]
    #[case(r#"-w "node_list""#, SbatchOption::NodeList("node_list".to_string()), r#"--nodelist="node_list""#)]
    #[case(r#"--nodes="nodes""#, SbatchOption::Nodes("nodes".to_string()), r#"--nodes="nodes""#)]
    #[case(r#"-N "nodes""#, SbatchOption::Nodes("nodes".to_string()), r#"--nodes="nodes""#)]
    #[case(r#"--ntasks=1"#, SbatchOption::NTasks(NonZeroU32::new(1).unwrap()), r#"--ntasks=1"#)]
    #[case(r#"--ntasks-per-core=1"#, SbatchOption::NTasksPerCore(NonZeroU32::new(1).unwrap()), r#"--ntasks-per-core=1"#)]
    #[case(r#"--ntasks-per-gpu=1"#, SbatchOption::NTasksPerGPU(NonZeroU32::new(1).unwrap()), r#"--ntasks-per-gpu=1"#)]
    #[case(r#"--ntasks-per-node=1"#, SbatchOption::NTasksPerNode(NonZeroU32::new(1).unwrap()), r#"--ntasks-per-node=1"#)]
    #[case(r#"--ntasks-per-socket=1"#, SbatchOption::NTasksPerSocket(NonZeroU32::new(1).unwrap()), r#"--ntasks-per-socket=1"#)]
    #[case(r#"--oom-kill-step="oom_kill_step""#, SbatchOption::OOMKillStep(Some("oom_kill_step".to_string())), r#"--oom-kill-step="oom_kill_step""#)]
    #[case(
        r#"--oom-kill-step"#,
        SbatchOption::OOMKillStep(None),
        r#"--oom-kill-step"#
    )]
    #[case(r#"--open-mode="open_mode""#, SbatchOption::OpenMode("open_mode".to_string()), r#"--open-mode="open_mode""#)]
    #[case(r#"--output="output""#, SbatchOption::Output("output".to_string()), r#"--output="output""#)]
    #[case(r#"-o "output""#, SbatchOption::Output("output".to_string()), r#"--output="output""#)]
    #[case(r#"--overcommit"#, SbatchOption::Overcommit, r#"--overcommit"#)]
    #[case(
        r#"--oversubscribe"#,
        SbatchOption::Oversubscribe,
        r#"--oversubscribe"#
    )]
    #[case(r#"--parsable"#, SbatchOption::Parsable, r#"--parsable"#)]
    #[case(r#"--partition="partition""#, SbatchOption::Partition("partition".to_string()), r#"--partition="partition""#)]
    #[case(r#"-p "partition""#, SbatchOption::Partition("partition".to_string()), r#"--partition="partition""#)]
    #[case(r#"--prefer="prefer""#, SbatchOption::Prefer("prefer".to_string()), r#"--prefer="prefer""#)]
    #[case(r#"--priority="priority""#, SbatchOption::Priority("priority".to_string()), r#"--priority="priority""#)]
    #[case(r#"--profile="profile""#, SbatchOption::Profile("profile".to_string()), r#"--profile="profile""#)]
    #[case(r#"--propagate="propagate""#, SbatchOption::Propagate(Some("propagate".to_string())), r#"--propagate="propagate""#)]
    #[case(r#"--propagate"#, SbatchOption::Propagate(None), r#"--propagate"#)]
    #[case(r#"--qos="qos""#, SbatchOption::Qos("qos".to_string()), r#"--qos="qos""#)]
    #[case(r#"-q "qos""#, SbatchOption::Qos("qos".to_string()), r#"--qos="qos""#)]
    #[case(r#"--quiet"#, SbatchOption::Quiet, r#"--quiet"#)]
    #[case(r#"--reboot"#, SbatchOption::Reboot, r#"--reboot"#)]
    #[case(r#"--requeue"#, SbatchOption::Requeue, r#"--requeue"#)]
    #[case(r#"--reservation="reservation""#, SbatchOption::Reservation("reservation".to_string()), r#"--reservation="reservation""#)]
    #[case(r#"--resv-ports="resv_ports""#, SbatchOption::ResvPorts(Some("resv_ports".to_string())), r#"--resv-ports="resv_ports""#)]
    #[case(r#"--resv-ports"#, SbatchOption::ResvPorts(None), r#"--resv-ports"#)]
    #[case(r#"--segment="segment""#, SbatchOption::Segment("segment".to_string()), r#"--segment="segment""#)]
    #[case(r#"--signal="signal""#, SbatchOption::Signal("signal".to_string()), r#"--signal="signal""#)]
    #[case(r#"--sockets-per-node="sockets_per_node""#, SbatchOption::SocketsPerNode("sockets_per_node".to_string()), r#"--sockets-per-node="sockets_per_node""#)]
    #[case(r#"--spread-job"#, SbatchOption::SpreadJob, r#"--spread-job"#)]
    #[case(r#"--stepmgr"#, SbatchOption::Stepmgr, r#"--stepmgr"#)]
    #[case(r#"--switches="switches""#, SbatchOption::Switches("switches".to_string()), r#"--switches="switches""#)]
    #[case(r#"--test-only"#, SbatchOption::TestOnly, r#"--test-only"#)]
    #[case(r#"--thread-spec=1"#, SbatchOption::ThreadSpec(NonZeroU32::new(1).unwrap()), r#"--thread-spec=1"#)]
    #[case(r#"--threads-per-core=1"#, SbatchOption::ThreadsPerCore(NonZeroU32::new(1).unwrap()), r#"--threads-per-core=1"#)]
    #[case(r#"--time="time""#, SbatchOption::Time("time".to_string()), r#"--time="time""#)]
    #[case(r#"-t "time""#, SbatchOption::Time("time".to_string()), r#"--time="time""#)]
    #[case(r#"--time-min="time_min""#, SbatchOption::TimeMin("time_min".to_string()), r#"--time-min="time_min""#)]
    #[case(r#"--tmp="tmp""#, SbatchOption::Tmp("tmp".to_string()), r#"--tmp="tmp""#)]
    #[case(r#"--tres-bind="tres_bind""#, SbatchOption::TresBind("tres_bind".to_string()), r#"--tres-bind="tres_bind""#)]
    #[case(r#"--tres-per-task="tres_per_task""#, SbatchOption::TresPerTask("tres_per_task".to_string()), r#"--tres-per-task="tres_per_task""#)]
    #[case(r#"--uid="uid""#, SbatchOption::UID("uid".to_string()), r#"--uid="uid""#)]
    #[case(r#"--usage"#, SbatchOption::Usage, r#"--usage"#)]
    #[case(r#"--use-min-nodes"#, SbatchOption::UseMinNodes, r#"--use-min-nodes"#)]
    #[case(r#"--verbose"#, SbatchOption::Verbose, r#"--verbose"#)]
    #[case(r#"--version"#, SbatchOption::Version, r#"--version"#)]
    #[case(r#"--wait"#, SbatchOption::Wait, r#"--wait"#)]
    #[case(r#"--wait-all-nodes="wait_all_nodes""#, SbatchOption::WaitAllNodes("wait_all_nodes".to_string()), r#"--wait-all-nodes="wait_all_nodes""#)]
    #[case(r#"--wckey="wc_key""#, SbatchOption::WCKey("wc_key".to_string()), r#"--wckey="wc_key""#)]
    #[case(r#"--wrap="wrap""#, SbatchOption::Wrap("wrap".to_string()), r#"--wrap="wrap""#)]
    fn test_new(
        #[case] input_string: &str,
        #[case] expected_enum: SbatchOption,
        #[case] expected_string: &str,
    ) {
        // Test creating a new SbatchOption
        let sbatch_option = SbatchOption::from_str(input_string);
        if !sbatch_option.is_ok() {
            println!("{:?}", sbatch_option);
        }
        assert!(sbatch_option.is_ok());
        let sbatch_option = sbatch_option.unwrap();

        // Test that the SbatchOption is the expected variant
        assert!(sbatch_option.is_same_variant(&expected_enum));

        // Test that the SbatchOption can be converted back to a string using Display
        assert_eq!(sbatch_option.to_string(), expected_string);
    }

    #[rstest]
    #[case(r#"--unknown="account""#)]
    #[case(r#"-u "hi"#)]
    #[case(r#"-u"#)]
    #[case(r#"--core-spec=hi"#)]
    #[case(r#"--cores-per-socket=hi"#)]
    #[case(r#"--cpus-per-gpu=hi"#)]
    #[case(r#"--cpus-per-task=hi"#)]
    #[case(r#"--delay-boot=hi"#)]
    #[case(r#"--min-cpus=hi"#)]
    #[case(r#"--ntasks=hi"#)]
    #[case(r#"--core-spec=-1"#)]
    #[case(r#"--cores-per-socket=-1"#)]
    #[case(r#"--cpus-per-gpu=-1"#)]
    #[case(r#"--cpus-per-task=-1"#)]
    #[case(r#"--delay-boot=-1"#)]
    #[case(r#"--min-cpus=-1"#)]
    #[case(r#"--ntasks=-1"#)]
    #[case(r#"--ntasks=0"#)]
    #[case(r#"--ntasks-per-core=0"#)]
    #[case(r#"--ntasks-per-gpu=0"#)]
    #[case(r#"--ntasks-per-node=0"#)]
    #[case(r#"--ntasks-per-socket=0"#)]
    #[case(r#"--thread-spec=0"#)]
    #[case(r#"--threads-per-core=0"#)]
    fn test_new_error(#[case] input_string: &str) {
        // Test creating a new SbatchOption
        let sbatch_option = SbatchOption::from_str(input_string);
        if !sbatch_option.is_err() {
            println!("{:?}", sbatch_option);
        }
        assert!(sbatch_option.is_err());

        // Test from_str
        let sbatch_option = SbatchOption::from_str(input_string);
        if !sbatch_option.is_err() {
            println!("{:?}", sbatch_option);
        }
        assert!(sbatch_option.is_err());
    }
}
