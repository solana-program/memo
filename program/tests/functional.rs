use {
    mollusk_svm::{result::Check, Mollusk},
    solana_account::Account,
    solana_instruction::{AccountMeta, Instruction},
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    spl_memo::*,
};

#[test]
fn test_memo_signing() {
    let memo = "🐆".as_bytes();
    let mollusk = Mollusk::new(&id(), "spl_memo");

    let first_address = Pubkey::new_unique();
    let second_address = Pubkey::new_unique();
    let third_address = Pubkey::new_unique();
    let pubkeys = [first_address, second_address, third_address];

    // Test complete signing
    let signer_key_refs: Vec<&Pubkey> = pubkeys.iter().collect();
    mollusk.process_and_validate_instruction(
        &build_memo(memo, &signer_key_refs),
        &[
            (first_address, Account::default()),
            (second_address, Account::default()),
            (third_address, Account::default()),
        ],
        &[Check::success()],
    );

    // Test unsigned memo
    mollusk.process_and_validate_instruction(&build_memo(memo, &[]), &[], &[Check::success()]);

    // Test missing signer(s)
    mollusk.process_and_validate_instruction(
        &Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new_readonly(first_address, true),
                AccountMeta::new_readonly(second_address, false),
                AccountMeta::new_readonly(third_address, true),
            ],
            data: memo.to_vec(),
        },
        &[
            (first_address, Account::default()),
            (second_address, Account::default()),
            (third_address, Account::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );

    mollusk.process_and_validate_instruction(
        &Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new_readonly(first_address, false),
                AccountMeta::new_readonly(second_address, false),
                AccountMeta::new_readonly(third_address, false),
            ],
            data: memo.to_vec(),
        },
        &[
            (first_address, Account::default()),
            (second_address, Account::default()),
            (third_address, Account::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );

    // Test invalid utf-8; demonstrate log
    let invalid_utf8 = [0xF0, 0x9F, 0x90, 0x86, 0xF0, 0x9F, 0xFF, 0x86];
    mollusk.process_and_validate_instruction(
        &build_memo(&invalid_utf8, &[]),
        &[],
        &[Check::err(ProgramError::InvalidInstructionData)],
    );
}

#[test]
fn test_memo_compute_limits() {
    let mollusk = Mollusk::new(&id(), "spl_memo");

    // Test memo length
    let mut memo = vec![];
    for _ in 0..1000 {
        let mut vec = vec![0x53, 0x4F, 0x4C];
        memo.append(&mut vec);
    }

    mollusk.process_and_validate_instruction(
        &build_memo(&memo[..450], &[]),
        &[],
        &[Check::success()],
    );

    mollusk.process_and_validate_instruction(
        &build_memo(&memo[..600], &[]),
        &[],
        &[Check::success(), Check::compute_units(8_503)],
    );

    let mut memo = vec![];
    for _ in 0..100 {
        let mut vec = vec![0xE2, 0x97, 0x8E];
        memo.append(&mut vec);
    }

    mollusk.process_and_validate_instruction(
        &build_memo(&memo[..60], &[]),
        &[],
        &[Check::success()],
    );

    mollusk.process_and_validate_instruction(
        &build_memo(&memo[..63], &[]),
        &[],
        &[Check::success(), Check::compute_units(62_887)],
    );

    // Test num signers with 32-byte memo
    let memo = [b'1'; 32];
    let mut pubkeys = vec![];
    for _ in 0..20 {
        pubkeys.push(Pubkey::new_unique());
    }
    let signer_key_refs: Vec<&Pubkey> = pubkeys.iter().collect();

    mollusk.process_and_validate_instruction(
        &build_memo(&memo, &signer_key_refs[..12]),
        pubkeys
            .iter()
            .take(12)
            .map(|k| (*k, Account::default()))
            .collect::<Vec<_>>()
            .as_slice(),
        &[Check::success()],
    );

    mollusk.process_and_validate_instruction(
        &build_memo(&memo, &signer_key_refs[..15]),
        pubkeys
            .iter()
            .take(15)
            .map(|k| (*k, Account::default()))
            .collect::<Vec<_>>()
            .as_slice(),
        &[Check::success(), Check::compute_units(31_788)],
    );
}
