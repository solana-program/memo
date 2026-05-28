#![deprecated(
    since = "6.1.0",
    note = "Please use `spl-memo-interface` for instruction types, and `pinocchio-memo-program` for the program implementation"
)]
#![deny(missing_docs)]

//! A program that accepts a string of encoded characters and verifies that it
//! parses, while verifying and logging signers. Currently handles UTF-8
//! characters.
//!
//! `⚠️ DEPRECATED`
//!
//! This crate has been deprecated and is no longer actively maintained.
//!
//! Please use [spl-memo-interface](https://crates.io/crates/spl-memo-interface) for
//! instruction types, and [pinocchio-memo-program](https://crates.io/crates/pinocchio-memo-program)
//! for the program implementation.

mod entrypoint;

// Process the input lazily.
lazy_program_entrypoint!(process_instruction);
// Disable the memory allocator.
no_allocator!();
// Use a `no_std` panic handler.
nostd_panic_handler!();
