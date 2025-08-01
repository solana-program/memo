use {
    solana_instruction::{AccountMeta, Instruction},
    solana_pubkey::Pubkey,
};

/// Build a memo instruction, possibly signed
///
/// Accounts expected by this instruction:
///
///   0. `..0+N` `[signer]` Expected signers; if zero provided, instruction will
///      be processed as a normal, unsigned spl-memo
pub fn build_memo(program_id: &Pubkey, memo: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: memo.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_memo() {
        let program_id = Pubkey::new_unique();
        let signer_pubkey = Pubkey::new_unique();
        let memo = "🐆".as_bytes();
        let instruction = build_memo(&program_id, memo, &[&signer_pubkey]);
        assert_eq!(memo, instruction.data);
        assert_eq!(instruction.accounts.len(), 1);
        assert_eq!(instruction.accounts[0].pubkey, signer_pubkey);
    }
}
