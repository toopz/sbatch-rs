use sbatch_rs::Sbatch;
use sbatch_rs::SbatchOption;

#[test]
fn test_set_script() {
    // Verify that it builds properly
    let sbatch = Sbatch::new()
        .set_script("test.sh".to_string())
        .unwrap()
        .build();
    assert!(sbatch.is_ok());
    assert_eq!(sbatch.unwrap(), "sbatch test.sh");
}

#[test]
fn test_set_script_bad_formatting() {
    // Verify that it builds properly
    let sbatch = Sbatch::new()
        .set_script(" test.sh ".to_string())
        .unwrap()
        .build();
    assert!(sbatch.is_ok());
    assert_eq!(sbatch.unwrap(), "sbatch test.sh");
}

#[test]
fn test_add_option() {
    // Verify that it builds properly
    let sbatch = Sbatch::new()
        .add_option(SbatchOption::JobName("test".to_string()))
        .unwrap()
        .add_option(SbatchOption::Output("test.out".to_string()))
        .unwrap()
        .add_option(SbatchOption::Error("test.err".to_string()))
        .unwrap()
        .set_script("test.sh".to_string())
        .unwrap()
        .build();
    assert!(sbatch.is_ok());

    // Extract the bash string
    assert_eq!(
        sbatch.unwrap(),
        "sbatch --error=test.err --job-name=test --output=test.out test.sh"
    );
}
