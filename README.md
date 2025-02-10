# Dangling Pointer Bug in Rust

This repository demonstrates a common error in Rust: creating a dangling pointer.  A dangling pointer occurs when a reference points to memory that has been deallocated or modified, leading to undefined behavior and potential crashes. The `bug.rs` file contains the buggy code, while `bugSolution.rs` provides a corrected version.

## How to Reproduce

1. Clone this repository.
2. Navigate to the repository's directory.
3. Compile and run `bug.rs` using `rustc bug.rs && ./bug`.
4. Observe the potential undefined behavior in the output.

## Solution

The solution involves careful management of lifetimes and references. The correct code is shown in `bugSolution.rs` and prevents the dangling pointer by ensuring that references remain valid.