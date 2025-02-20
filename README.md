# sbatch-rs
Sbatch interface in rust

## Overview
This crate provides a simple interface to SLURM's sbatch command.
The primary goal of this crate is to provide a safe and ergonomic way to interact with sbatch from rust.
Internally, the crate provides numerous checks to ensure that each sbatch option is valid
and that the resulting sbatch command is called with the correct arguments. When faced with
an implementation decision, the crate will always err on the side of safety and correctness
over performance or user convenience.

All known sbatch options are defined as enums, and the user cannot provide an invalid option to the
sbatch command. Furthermore, these enums store Strings, NonZeroU32s, or other "safe"
types, which prevent the user from accidentally providing a non-sensical value to sbatch, e.g.,
a negative or zero number for the number of nodes.

This crate also provides an interface to specify sbatch dependencies. The user can
specify each dependency as a separate enum variant, and the dependent jobid is checked to ensure that
is is either a non-zero u32, or a bash variable name such as $job_id1.
