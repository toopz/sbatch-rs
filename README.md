# sbatch-rs
Sbatch command generator in rust

## Overview
This is a simple library which allows you to generate sbatch commands in rust in a programmatic way with error checking.

### SbatchOption
The SbatchOption enum is a complete enumeration of all the options that can be passed to sbatch.
For example:

```rust
use sbatch_rs::SbatchOption;

let job_name = SbatchOption::JobName("my_job".to_string());
let cpu = SbatchOption::NTasks("4".to_string());
let time = SbatchOption::Time("1:00:00".to_string());
```

Each SbatchOption takes either Nothing, a String, or an Option<Sting> as an argument. The rational
against using say u32 for the number of tasks is that it would prevent the use of variables.
For example, this is valid:

```rust
use sbatch_rs::SbatchOption;

let cpu = SbatchOption::NTasks("${NODE_CORES}".to_string());
```

### Sbatch
The Sbatch struct is a builder which allows you to build up an sbatch command with error checking.

```rust
use sbatch_rs::{Sbatch, SbatchOption};

// Create a new `Sbatch` instance
let sbatch = Sbatch::new()
     .add_option(SbatchOption::JobName("test".to_string())).unwrap()
     .add_option(SbatchOption::Output("test.out".to_string())).unwrap()
     .add_option(SbatchOption::Error("test.err".to_string())).unwrap()
     .set_script("test.sh".to_string()).unwrap()
     .build();

// Verify that the `sbatch` command was built properly
assert!(sbatch.is_ok());
assert_eq!(sbatch.unwrap(), "sbatch --error=test.err --job-name=test --output=test.out test.sh");
```

### Dependency
For complex dependencies, you can use the Dependency struct to build up a dependency string,
which can then be used to create a SbatchOption::Dependency option.

```rust

use sbatch_rs::{Dependency, DependencyType};

// Create a new `And` dependency
let mut dependency = Dependency::new_and();
    .push(DependencyType::After("123".to_string()))? // Using the DependencyType enum
    .push_after_time_delay("456", "10").unwrap()?    // Using the helper function
    .build()?;
assert_eq!(dependency, "after:123,after:456+10");

// Now use the string to create a SbatchOption::Dependency
let sbatch_dependency = SbatchOption::Dependency(dependency);
```