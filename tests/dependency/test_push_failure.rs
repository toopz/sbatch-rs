use rstest::rstest;
use sbatch_rs::{Dependency, DependencyType};

#[rstest]
#[case("")]
#[case("123 ")]
#[case("123  ")]
#[case(" 123")]
#[case(" 123 ")]
fn test_push_error(#[case] s: &str) {
    let mut dependency = Dependency::new_and();
    let dependency_result = dependency.push(DependencyType::After(s.to_string()));
    assert!(dependency_result.is_err());
}
