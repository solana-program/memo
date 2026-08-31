#![no_std]
#![deny(missing_docs)]

//! An interface for programs that accept a string of encoded characters and
//! verifies that it parses, while verifying and logging signers.

/// CPI interface for invoking the Memo program.
#[cfg(feature = "cpi")]
pub mod cpi;

/// Instruction definition for the Memo program.
#[cfg(feature = "instruction")]
pub mod instruction;

/// Legacy symbols from Memo version 1
pub mod v1 {
    solana_address::declare_id!("Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo");
}

/// Symbols from Memo version 3
pub mod v3 {
    solana_address::declare_id!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
}

/// Symbols from Memo version 4
pub mod v4 {
    solana_address::declare_id!("Memo4c2pN8afCj432Lb7RMVKi9PbQnnW7ewFFaV3oAH");
}
