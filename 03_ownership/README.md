# 03_ownership

Based on [Chapter 4](https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html) examples from the official Rust documentation.

This is a simple program to show basic information about Rust ownership
feature.

## Theory

### Ownership Rules

  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.

### The Rules of References

  - At any given time, you can have either one mutable reference or any number of immutable references.
  - References must always be valid.

## Starting

```bash
# compile the program and download required dependencies
cargo build

# run
cargo run

# optional: build documentation of dependencies in use
cargo doc --open
```
