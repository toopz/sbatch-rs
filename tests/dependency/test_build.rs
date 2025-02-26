use rstest::rstest;
use sbatch_rs::{Dependency, DependencyType};

#[rstest]
#[case(Dependency::And(vec![DependencyType::After("123".to_string())]), "after:123")]
#[case(Dependency::Or(vec![DependencyType::After("123".to_string())]), "after:123")]
#[case(Dependency::And(vec![DependencyType::After("123".to_string()), DependencyType::After("456".to_string())]), "after:123,after:456")]
#[case(Dependency::Or(vec![DependencyType::After("123".to_string()), DependencyType::After("456".to_string())]), "after:123?after:456")]
#[case(Dependency::And(vec![DependencyType::After("123".to_string()), DependencyType::After("456".to_string()), DependencyType::After("789".to_string())]), "after:123,after:456,after:789")]
#[case(Dependency::Or(vec![DependencyType::After("123".to_string()), DependencyType::After("456".to_string()), DependencyType::After("789".to_string())]), "after:123?after:456?after:789")]
fn test_build(#[case] dependency: Dependency, #[case] expected: &str) {
    assert_eq!(dependency.build().unwrap(), expected);
}

#[rstest]
#[case(Dependency::And(vec![DependencyType::After("123".to_string())]), "after:123")]
#[case(Dependency::Or(vec![DependencyType::After("123".to_string())]), "after:123")]
#[case(Dependency::And(vec![DependencyType::After("123".to_string()), DependencyType::After("456".to_string())]), "after:123,after:456")]
#[case(Dependency::Or(vec![DependencyType::After("123".to_string()), DependencyType::After("456".to_string())]), "after:123?after:456")]
#[case(Dependency::And(vec![DependencyType::After("456".to_string()), DependencyType::After("123".to_string())]), "after:123,after:456")]
#[case(Dependency::Or(vec![DependencyType::After("456".to_string()), DependencyType::After("123".to_string())]), "after:123?after:456")]
#[case(Dependency::Or(vec![DependencyType::After("456".to_string()), DependencyType::AfterOk("123".to_string())]), "after:456?afterok:123")]
#[case(Dependency::Or(vec![DependencyType::AfterOk("123".to_string()), DependencyType::After("456".to_string())]), "after:456?afterok:123")]
#[case(Dependency::Or(vec![DependencyType::After("456".to_string()), DependencyType::Singleton]), "after:456?singleton")]
fn test_build_order(#[case] dependency: Dependency, #[case] expected: &str) {
    assert_eq!(dependency.build().unwrap(), expected);
}

#[rstest]
#[case(Dependency::And(vec![]))]
#[case(Dependency::Or(vec![]))]
#[case(Dependency::And(vec![DependencyType::After("123  ".to_string())]))]
#[case(Dependency::Or(vec![DependencyType::After("123  ".to_string())]))]
fn test_build_error(#[case] dependency: Dependency) {
    assert!(dependency.build().is_err());
}
