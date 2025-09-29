//! A pinocchio-based Memo (aka 'p-memo') program.
//!
//! The Memo program is a simple program that validates a string of UTF-8 encoded
//! characters and verifies that any accounts provided are signers of the transaction.
//! The program also logs the memo, as well as any verified signer addresses, to the
//! transaction log, so that anyone can easily observe memos and know they were
//! approved by zero or more addresses by inspecting the transaction log from a
//! trusted provider.

#![no_std]

mod entrypoint;

use pinocchio::{lazy_program_entrypoint, no_allocator, nostd_panic_handler};

use crate::entrypoint::process_instruction;

// Process the input lazily.
lazy_program_entrypoint!(process_instruction);
// Disable the memory allocator.
no_allocator!();
// Use a `no_std` panic handler.
nostd_panic_handler!();
