#![deny(missing_docs)]

//! A program that accepts a string of encoded characters and verifies that it
//! parses, while verifying and logging signers. Currently handles UTF-8
//! characters.

mod entrypoint;
pub mod processor;

// Export current sdk types for downstream users building with a different sdk
// version
pub use {
    solana_account_info, solana_instruction, solana_msg, solana_program_entrypoint,
    solana_program_error, solana_pubkey,
    spl_memo_interface::{
        v1,
        v3::{check_id, id, ID},
    },
};
use {solana_instruction::Instruction, solana_pubkey::Pubkey};

/// Build a memo instruction, possibly signed
///
/// Accounts expected by this instruction:
///
///   0. `..0+N` `[signer]` Expected signers; if zero provided, instruction will
///      be processed as a normal, unsigned spl-memo
pub fn build_memo(memo: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    spl_memo_interface::instruction::build_memo(&id(), memo, signer_pubkeys)
}
