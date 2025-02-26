use rstest::rstest;
use sbatch_rs::Sbatch;
use sbatch_rs::SbatchOption;

#[test]
fn test_new_and_default() {
    let sbatch = Sbatch::new();
    assert!(sbatch.build().is_err());

    let sbatch = Sbatch::default();
    assert!(sbatch.build().is_err());
}

#[test]
fn test_build_add_option() {
    let sbatch = Sbatch::new()
        .add_option(SbatchOption::JobName("test".to_string()))
        .unwrap()
        .add_option(SbatchOption::Output("test.out".to_string()))
        .unwrap()
        .add_option(SbatchOption::Error("test.err".to_string()))
        .unwrap()
        .build();

    assert!(sbatch.is_ok());
}

#[test]
fn test_build_set_script() {
    let sbatch = Sbatch::new()
        .set_script("echo test".to_string())
        .unwrap()
        .build();

    assert!(sbatch.is_ok());
}

#[rstest]
#[case("")]
#[case(" ")]
#[case("  ")]
#[case("   ")]
fn test_build_set_script_errors(#[case] script: &str) {
    let mut sbatch = Sbatch::new();
    let set_result = sbatch.set_script(script.to_string());
    assert!(set_result.is_err());
}

#[test]
fn test_build_error_empty() {
    let sbatch = Sbatch::new().build();
    assert!(sbatch.is_err());
}
