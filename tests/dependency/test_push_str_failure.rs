use sbatch_rs::Dependency;

#[test]
fn test_push_after_error() {
    let mut dependency = Dependency::new_and();
    let dependency = dependency.push_after("123 ");
    assert!(dependency.is_err());
}

#[test]
fn test_push_after_time_delay_error() {
    let mut dependency = Dependency::new_and();
    let dependency = dependency.push_after_time_delay("123", "1 ");
    assert!(dependency.is_err());
}

#[test]
fn test_push_after_any_error() {
    let mut dependency = Dependency::new_and();
    let dependency = dependency.push_after_any("123 ");
    assert!(dependency.is_err());
}

#[test]
fn test_push_after_burst_buffer_error() {
    let mut dependency = Dependency::new_and();
    let dependency = dependency.push_after_burst_buffer("123 ");
    assert!(dependency.is_err());
}

#[test]
fn test_push_after_corr_error() {
    let mut dependency = Dependency::new_and();
    let dependency = dependency.push_after_corr("123 ");
    assert!(dependency.is_err());
}

#[test]
fn test_push_after_not_ok_error() {
    let mut dependency = Dependency::new_and();
    let dependency = dependency.push_after_not_ok("123 ");
    assert!(dependency.is_err());
}

#[test]
fn test_push_after_ok_error() {
    let mut dependency = Dependency::new_and();
    let dependency = dependency.push_after_ok("123 ");
    assert!(dependency.is_err());
}
