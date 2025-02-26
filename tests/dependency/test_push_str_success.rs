use sbatch_rs::Dependency;

#[test]
fn test_push_after() {
    let dependency = Dependency::new_and()
        .push_after("123")
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "after:123");
}

#[test]
fn test_push_after_time_delay() {
    let dependency = Dependency::new_and()
        .push_after_time_delay("123", "1")
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "after:123+1");
}

#[test]
fn test_push_after_any() {
    let dependency = Dependency::new_and()
        .push_after_any("123")
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "afterany:123");
}

#[test]
fn test_push_after_burst_buffer() {
    let dependency = Dependency::new_and()
        .push_after_burst_buffer("123")
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "afterburstbuffer:123");
}

#[test]
fn test_push_after_corr() {
    let dependency = Dependency::new_and()
        .push_after_corr("123")
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "aftercorr:123");
}

#[test]
fn test_push_after_not_ok() {
    let dependency = Dependency::new_and()
        .push_after_not_ok("123")
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "afternotok:123");
}

#[test]
fn test_push_after_ok() {
    let dependency = Dependency::new_and()
        .push_after_ok("123")
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "afterok:123");
}

#[test]
fn test_push_singleton() {
    let dependency = Dependency::new_and()
        .push_singleton()
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(dependency, "singleton");
}
