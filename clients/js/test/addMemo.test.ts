import { getBase58Encoder, getUtf8Decoder } from '@solana/kit';
import test from 'ava';
import { MEMO_PROGRAM_ADDRESS } from '../src';
import { createTestClient } from './_setup';

test('it adds custom text to the transaction logs', async t => {
    // Given a client with an airdropped payer.
    const client = await createTestClient();

    // When the payer sends a transaction with a custom memo.
    const {
        context: { signature },
    } = await client.memo.instructions.addMemo({ memo: 'Hello world!' }).sendTransaction();

    // Then the instruction data of the memo instruction contains our memo text.
    const result = await client.rpc
        .getTransaction(signature, {
            encoding: 'json',
            maxSupportedTransactionVersion: 0,
        })
        .send();
    const { accountKeys, instructions } = result!.transaction.message;
    const memoInstruction = instructions.find(ix => accountKeys[ix.programIdIndex] === MEMO_PROGRAM_ADDRESS);
    t.truthy(memoInstruction, 'expected a memo instruction in the transaction');
    const instructionDataBytes = getBase58Encoder().encode(memoInstruction!.data);
    const instructionMemo = getUtf8Decoder().decode(instructionDataBytes);
    t.is(instructionMemo, 'Hello world!');
});
