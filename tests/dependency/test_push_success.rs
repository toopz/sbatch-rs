use sbatch_rs::{Dependency, DependencyType};

#[test]
fn test_push() {
    let dependency = Dependency::new_and()
        .push(DependencyType::After("123".to_string()))
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "after:123");

    let dependency = Dependency::new_or()
        .push(DependencyType::After("123".to_string()))
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "after:123");
}

#[test]
fn test_push_complex() {
    let dependency = Dependency::new_and()
        .push(DependencyType::After("123".to_string()))
        .unwrap()
        .push(DependencyType::AfterOk("456".to_string()))
        .unwrap()
        .push(DependencyType::Singleton)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "after:123,afterok:456,singleton");
}
