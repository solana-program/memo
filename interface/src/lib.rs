#![deny(missing_docs)]

//! An interface for programs that accept a string of encoded characters and
//! verifies that it parses, while verifying and logging signers.

/// Instruction type
pub mod instruction;

/// Legacy symbols from Memo version 1
pub mod v1 {
    solana_pubkey::declare_id!("Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo");
}

/// Symbols from Memo version 3
pub mod v3 {
    solana_pubkey::declare_id!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
}
