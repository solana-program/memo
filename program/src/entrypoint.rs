use {
    pinocchio::{error::ProgramError, AccountView, Address, ProgramResult},
    solana_program_log::log,
};

/// Process a memo instruction.
///
/// This function processes a memo instruction by logging the public keys of the signers
/// (if any) and the memo data itself, checking if the required signatures are present;
/// when any required signature is missing, it returns `ProgramError::MissingRequiredSignature`
/// error.
pub fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    // Validates signer accounts (if any).

    if !accounts.is_empty() {
        let mut missing_required_signature = false;
        // Logs a message indicating that there are signers.
        log!("Signed by:");

        for account in accounts {
            if account.is_signer() {
                // SAFETY: Use the syscall to log the public key of the account.
                #[cfg(any(target_os = "solana", target_arch = "bpf"))]
                unsafe {
                    pinocchio::syscalls::sol_log_pubkey(account.address().as_array().as_ptr())
                };
            } else {
                missing_required_signature = true;
            }
        }

        if missing_required_signature {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }

    // Logs the length of the memo message and its content.
    log!("Memo (len {})", instruction_data.len());

    // SAFETY: The syscall will validate the UTF-8 encoding of the memo data.
    #[cfg(any(target_os = "solana", target_arch = "bpf"))]
    unsafe {
        pinocchio::syscalls::sol_log_(instruction_data.as_ptr(), instruction_data.len() as u64);
    }

    Ok(())
}
