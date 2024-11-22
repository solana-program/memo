import { Buffer } from 'buffer';
import { PublicKey, TransactionInstruction } from '@solana/web3.js';

export const MEMO_PROGRAM_ID: PublicKey = new PublicKey('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr');

/**
 * Creates and returns an instruction which validates a string of UTF-8
 * encoded characters and verifies that any accounts provided are signers of
 * the transaction.  The program also logs the memo, as well as any verified
 * signer addresses, to the transaction log, so that anyone can easily observe
 * memos and know they were approved by zero or more addresses by inspecting
 * the transaction log from a trusted provider.
 *
 * Public keys passed in via the signerPubkeys will identify Signers which
 * must subsequently sign the Transaction including the returned
 * TransactionInstruction in order for the transaction to be valid.
 *
 * @param memo The UTF-8 encoded memo string to validate
 * @param signerPubkeys An array of public keys which must sign the
 *        Transaction including the returned TransactionInstruction in order
 *        for the transaction to be valid and the memo verification to
 *        succeed.  null is allowed if there are no signers for the memo
 *        verification.
 **/
export function createMemoInstruction(memo: string, signerPubkeys?: Array<PublicKey>): TransactionInstruction {
    const keys =
        signerPubkeys == null
            ? []
            : signerPubkeys.map(function (key) {
                  return { pubkey: key, isSigner: true, isWritable: false };
              });

    return new TransactionInstruction({
        keys: keys,
        programId: MEMO_PROGRAM_ID,
        data: Buffer.from(memo, 'utf8'),
    });
}
