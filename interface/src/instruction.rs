use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};

/// Build a memo instruction, possibly signed.
///
/// Accounts expected by this instruction:
///
///   0. `..0+N` `[signer]` Expected signers; if zero provided, instruction will
///      be processed as a normal, unsigned spl-memo
pub fn memo(program_id: &Address, message: &[u8], signer_pubkeys: &[&Address]) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: message.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memo() {
        let program_id = Address::new_unique();
        let signer_pubkey = Address::new_unique();
        let message = "🐆".as_bytes();
        let instruction = memo(&program_id, message, &[&signer_pubkey]);

        assert_eq!(message, instruction.data);
        assert_eq!(instruction.accounts.len(), 1);
        assert_eq!(instruction.accounts[0].pubkey, signer_pubkey);
    }
}
