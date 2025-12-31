use pinocchio::{
    entrypoint::{InstructionContext, MaybeAccount},
    program_error::ProgramError,
    syscalls::{sol_log_, sol_log_pubkey},
    ProgramResult,
};

/// Process a memo instruction.
///
/// This function processes a memo instruction by logging the public keys of the signers
/// (if any) and the memo data itself, checking if the required signatures are present;
/// when any required signature is missing, it returns `ProgramError::MissingRequiredSignature`
/// error.
pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    let mut missing_required_signature = false;

    if context.remaining() > 0 {
        const SIGNED_BY_MSG: &[u8] = b"Signed by:";
        // SAFETY: Logging a constant string with known valid length.
        unsafe {
            sol_log_(SIGNED_BY_MSG.as_ptr(), SIGNED_BY_MSG.len() as u64);
        }

        while context.remaining() > 0 {
            // Duplicated accounts are implicitly checked since at least one of the
            // "copies" must be a signer.
            if let MaybeAccount::Account(account) = context.next_account()? {
                if account.is_signer() {
                    // SAFETY: Use the syscall to log the public key of the account.
                    unsafe { sol_log_pubkey(account.key().as_ptr()) };
                } else {
                    missing_required_signature = true;
                }
            }
        }

        if missing_required_signature {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }

    let instruction_data = context.instruction_data()?;

    let _memo =
        core::str::from_utf8(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;

    // SAFETY: UTF-8 validation passed, syscall will succeed.
    unsafe {
        sol_log_(instruction_data.as_ptr(), instruction_data.len() as u64);
    }

    Ok(())
}
