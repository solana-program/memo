import { getUtf8Decoder } from '@solana/kit';
import { expect, it } from 'vitest';

import { MEMO_PROGRAM_ADDRESS } from '../src';
import { createTestClient } from './_setup';

it('adds custom text to the transaction logs', async () => {
    // Given a client with an airdropped payer.
    const client = await createTestClient();

    // When the payer sends a transaction with a custom memo.
    const {
        context: { message, signature },
    } = await client.memo.instructions.addMemo({ memo: 'Hello world!' }).sendTransaction();

    // Then the planned message contains a memo instruction carrying our memo text.
    const memoInstruction = message?.instructions.find(ix => ix.programAddress === MEMO_PROGRAM_ADDRESS);
    expect(memoInstruction).toBeTruthy();
    expect(memoInstruction?.data).toBeDefined();
    expect(getUtf8Decoder().decode(memoInstruction!.data!)).toBe('Hello world!');

    // And the on-chain transaction logs include the memo text.
    const tx = client.svm.getTransaction(signature) as { logs: () => string[] } | null;
    expect(tx).not.toBeNull();
    expect(tx!.logs()).toEqual(
        expect.arrayContaining([expect.stringContaining('Memo (len 12)'), expect.stringContaining('Hello world!')]),
    );
});
