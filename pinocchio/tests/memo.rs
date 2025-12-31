use mollusk_svm::{result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

/// Program ID for the p-memo program.
const PROGRAM_ID: Pubkey = Pubkey::from_str_const("PMemo11111111111111111111111111111111111111");

/// The memo to be printed.
const MEMO: &str = "why does spl memo use 36000 cus to print len 60 msg of ascii";

/// Creates an instruction for the p-memo program.
fn instruction(message: &[u8], signers: Option<&[Pubkey]>) -> Instruction {
    let accounts = if let Some(signers) = signers {
        let mut accounts = Vec::with_capacity(signers.len());
        for signer in signers {
            accounts.push(AccountMeta::new_readonly(*signer, true));
        }
        accounts
    } else {
        Vec::new()
    };

    Instruction {
        program_id: PROGRAM_ID,
        accounts,
        data: message.to_vec(),
    }
}

#[test]
fn test_valid_ascii_no_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let instruction = instruction(MEMO.as_bytes(), None);

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}

#[test]
fn fail_test_invalid_ascii_no_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let instruction = instruction(&[255, 255], None);

    mollusk.process_and_validate_instruction(
        &instruction,
        &[],
        &[Check::err(ProgramError::InvalidInstructionData)],
    );
}

#[test]
fn test_valid_ascii_one_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let signer = Pubkey::new_unique();
    let instruction = instruction(MEMO.as_bytes(), Some(&[signer]));

    mollusk.process_and_validate_instruction(
        &instruction,
        &[(signer, Account::default())],
        &[Check::success()],
    );
}

#[test]
fn fail_test_invalid_missing_signer() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let signer = Pubkey::new_unique();
    let mut instruction = instruction(MEMO.as_bytes(), Some(&[signer]));
    // Set the account to be non-signer.
    instruction.accounts[0].is_signer = false;

    mollusk.process_and_validate_instruction(
        &instruction,
        &[(signer, Account::default())],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn test_valid_ascii_two_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let signers = [Pubkey::new_unique(), Pubkey::new_unique()];
    let instruction = instruction(MEMO.as_bytes(), Some(&signers));

    mollusk.process_and_validate_instruction(
        &instruction,
        &signers
            .iter()
            .map(|signer| (*signer, Account::default()))
            .collect::<Vec<(Pubkey, Account)>>(),
        &[Check::success()],
    );
}

#[test]
fn test_valid_ascii_three_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let signers = [
        Pubkey::new_unique(),
        Pubkey::new_unique(),
        Pubkey::new_unique(),
    ];
    let instruction = instruction(MEMO.as_bytes(), Some(&signers));

    mollusk.process_and_validate_instruction(
        &instruction,
        &signers
            .iter()
            .map(|signer| (*signer, Account::default()))
            .collect::<Vec<(Pubkey, Account)>>(),
        &[Check::success()],
    );
}

#[test]
fn test_valid_ascii_duplicated_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let unique = Pubkey::new_unique();
    let duplicated = Pubkey::new_unique();
    let instruction = instruction(MEMO.as_bytes(), Some(&[duplicated, unique, duplicated]));

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (duplicated, Account::default()),
            (unique, Account::default()),
        ],
        &[Check::success()],
    );
}

#[test]
fn test_valid_empty_memo() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let instruction = instruction(&[], None);

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}

#[test]
fn test_valid_max_size_memo() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    // Maximum transaction MTU is 1232 bytes.
    let max_memo = vec![b'A'; 1232];
    let instruction = instruction(&max_memo, None);

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}

#[test]
fn test_invalid_utf8_error_code() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    // Invalid UTF-8 sequence.
    let invalid_utf8 = [0xF0, 0x9F, 0xFF, 0x86];
    let instruction = instruction(&invalid_utf8, None);

    // Verify that we return InvalidInstructionData, not ProgramFailedToComplete.
    mollusk.process_and_validate_instruction(
        &instruction,
        &[],
        &[Check::err(ProgramError::InvalidInstructionData)],
    );
}
