# Rust Mutable Borrow Error Example

This repository contains a simple example demonstrating a common error in Rust when dealing with mutable and immutable borrows. The `bug.rs` file shows the erroneous code, while `bugSolution.rs` provides a solution to fix the error.

The core issue lies in trying to have both a mutable reference (`&mut x`) and an immutable reference (`&x`) to the same variable (`x`) simultaneously. Rust's borrow checker prevents this to avoid data races and other concurrency issues.

To resolve this, you will need to refactor the code to ensure that only one type of borrow exists at any given time.  See `bugSolution.rs` for different ways to achieve this.