use {
    core::{mem::MaybeUninit, slice::from_raw_parts},
    solana_account_view::AccountView,
    solana_address::Address,
    solana_instruction_view::{
        cpi::{invoke_signed_unchecked, CpiAccount, Signer, MAX_STATIC_CPI_ACCOUNTS},
        InstructionAccount, InstructionView,
    },
    solana_program_error::{ProgramError, ProgramResult},
};

/// Writes a message to the transaction log, validating that
/// provided accounts are signers.
///
/// ### Accounts:
///   0. `..+N` `[SIGNER]` N signing accounts
pub struct Memo<'memo, 'signers, S: AsRef<AccountView>> {
    /// The message to log.
    pub message: &'memo str,

    /// Signing accounts.
    pub signers: &'signers [S],

    /// The Memo program to invoke.
    pub program_id: &'static Address,
}

impl<S: AsRef<AccountView>> Memo<'_, '_, S> {
    /// Invokes the Memo program with the provided message and signing accounts.
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    /// Invokes the Memo program with the provided message and signing accounts.
    ///
    /// Seeds for signing accounts can be provided via `signers` parameter.
    #[inline(always)]
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let mut instruction_accounts =
            [const { MaybeUninit::<InstructionAccount>::uninit() }; MAX_STATIC_CPI_ACCOUNTS];

        let expected_account = self.signers.len();

        if expected_account > MAX_STATIC_CPI_ACCOUNTS {
            return Err(ProgramError::InvalidArgument);
        }

        let mut accounts = [const { MaybeUninit::<CpiAccount>::uninit() }; MAX_STATIC_CPI_ACCOUNTS];

        for i in 0..expected_account {
            // SAFETY: `expected_account` is less than MAX_STATIC_CPI_ACCOUNTS.
            unsafe {
                let signer = self.signers.get_unchecked(i);

                instruction_accounts.get_unchecked_mut(i).write(
                    InstructionAccount::readonly_signer(signer.as_ref().address()),
                );

                CpiAccount::init_from_account_view(signer.as_ref(), accounts.get_unchecked_mut(i));
            }
        }

        // SAFETY: both `instruction_accounts` and `accounts` are initialized.
        unsafe {
            invoke_signed_unchecked(
                &InstructionView {
                    program_id: self.program_id,
                    accounts: from_raw_parts(instruction_accounts.as_ptr() as _, expected_account),
                    data: self.message.as_bytes(),
                },
                from_raw_parts(accounts.as_ptr() as _, expected_account),
                signers,
            );
        }

        Ok(())
    }
}
