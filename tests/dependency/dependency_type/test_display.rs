use rstest::rstest;
pub use sbatch_rs::DependencyType;

#[rstest]
#[case(DependencyType::After("123".to_string()), "after:123")]
#[case(DependencyType::AfterTimeDelay("123".to_string(), "10".to_string()), "after:123+10")]
#[case(DependencyType::AfterAny("123".to_string()), "afterany:123")]
#[case(DependencyType::AfterBurstBuffer("123".to_string()), "afterburstbuffer:123")]
#[case(DependencyType::AfterCorr("123".to_string()), "aftercorr:123")]
#[case(DependencyType::AfterNotOk("123".to_string()), "afternotok:123")]
#[case(DependencyType::AfterOk("123".to_string()), "afterok:123")]
#[case(DependencyType::Singleton, "singleton")]
#[case(DependencyType::After("${job_id}".to_string()), "after:${job_id}")]
#[case(DependencyType::AfterTimeDelay("${job_id}".to_string(), "${time}".to_string()), "after:${job_id}+${time}")]
#[case(DependencyType::AfterAny("${job_id}".to_string()), "afterany:${job_id}")]
#[case(DependencyType::AfterBurstBuffer("${job_id}".to_string()), "afterburstbuffer:${job_id}")]
#[case(DependencyType::AfterCorr("${job_id}".to_string()), "aftercorr:${job_id}")]
#[case(DependencyType::AfterNotOk("${job_id}".to_string()), "afternotok:${job_id}")]
#[case(DependencyType::AfterOk("${job_id}".to_string()), "afterok:${job_id}")]
fn test_dependency_type_to_string(#[case] dependency: DependencyType, #[case] expected: &str) {
    assert_eq!(dependency.to_string(), expected);
}
